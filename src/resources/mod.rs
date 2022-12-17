use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Mutex;
use actix_web::{HttpRequest, HttpResponse, HttpResponseBuilder, Resource, Responder, web};
use actix_web::http::{Method, StatusCode};
use actix_web::http::header::HeaderValue;
use actix_web::web::{Data, resource};
use serde::de::Unexpected::Map;
use crate::model::*;
use crate::util::decode_jwt;


fn generate_handlers() -> Vec<(String, String, actix_web::Route, Route)> {
    let raw = std::fs::read_to_string("./api-config.json").unwrap();
    let root: Root = serde_json::from_str(raw.as_str()).unwrap();
    log::info!("root : {:?}", root);
    let mut resources = vec![];
    for route in root.routes.iter() {
        let method = route.method.clone().to_uppercase();
        let http_route = actix_web::Route::new()
            .method(Method::from_str(method.as_str()).unwrap())
            .to(handler);
        resources.push((route.method.clone().to_uppercase(), route.endpoint.clone(), http_route, route.clone()));
    }
    resources
}

async fn handler(req: HttpRequest, route_map: Data<HashMap<String, Route>>) -> impl Responder {
    log::info!("request method: {}", req.method());
    log::info!("request path: {}", req.path());
    log::info!("route map: {:?}", route_map);
    let route = route_map.get(format!("{}-{}", req.method().to_string().clone(), req.path().to_string()).as_str()).unwrap();

    match req.headers().get("authorization") {
        None => {
            HttpResponse::with_body(StatusCode::from_u16(route.responses.get(0).unwrap().status_code).unwrap(), format!(""))
        }
        Some(bearer) => {
            let mut r = route.responses.get(0).unwrap().clone();
            let claim = decode_jwt(Some(format!("{}", bearer.to_str().unwrap())));
            for response in &route.responses {
                if response.file_path.contains(&claim.user()) {
                    r = response.clone();
                    break;
                }
            }

            if r.status_code >=200 && r.status_code < 400 {
                let json = std::fs::read_to_string(r.file_path).unwrap_or(r.body);
                HttpResponse::with_body(StatusCode::from_u16(r.status_code).unwrap(), json)
            } else {
                HttpResponse::with_body(StatusCode::from_u16(r.status_code).unwrap(), format!(""))
            }
        }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    let mut scope = web::scope("");
    let mut route_map = HashMap::new();
    for handler in generate_handlers().into_iter() {
        scope = scope.route(handler.1.as_str(), handler.2);
        route_map.insert(format!("{}-{}", handler.0.clone(), format!("/{}", handler.1.clone())), handler.3.clone());
    }
    scope = scope.app_data(Data::new(route_map));
    cfg.service(scope);
}