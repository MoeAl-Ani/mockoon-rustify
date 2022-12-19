use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use actix_web::{HttpRequest, HttpResponse, HttpResponseBuilder, Resource, Responder, web};
use actix_web::http::{Method, StatusCode};
use actix_web::http::header::{HeaderMap, HeaderName, HeaderValue};
use actix_web::web::{Data, resource};
use serde::de::Unexpected::Map;
use crate::model::*;
use crate::util::{decode_jwt, extract_config_path};


fn generate_handlers(api_config_path: String) -> Vec<(String, String, actix_web::Route, Route)> {
    let raw = std::fs::read_to_string(api_config_path).unwrap();
    let mut root: Root = serde_json::from_str(raw.as_str()).unwrap();
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

async fn handler(req: HttpRequest, route_meta: Data<RouteMeta>) -> impl Responder {
    log::info!("request method: {}", req.method());
    log::info!("request path: {}", req.path());
    log::info!("route map: {:?}", route_meta.1);
    let route = route_meta.1.get(format!("{}-{}", req.method().to_string().clone(), req.path().to_string()).as_str()).unwrap();

    match req.headers().get("authorization") {
        None => {
            let mut response = HttpResponse::with_body(StatusCode::from_u16(route.responses.get(0).unwrap().status_code).unwrap(), format!(""));
            write_headers(response.headers_mut(), route.responses.get(0).unwrap().clone());
            response
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
                let file_path = PathBuf::from(route_meta.0.clone()).join(r.file_path.clone()).canonicalize().unwrap();
                let json = std::fs::read_to_string(file_path).unwrap_or(r.body.clone());
                let mut response = HttpResponse::with_body(StatusCode::from_u16(r.status_code.clone()).unwrap(), json);
                write_headers(response.headers_mut(), r.clone());
                response
            } else {
                let mut response = HttpResponse::with_body(StatusCode::from_u16(r.status_code.clone()).unwrap(), format!(""));;
                write_headers(response.headers_mut(), r.clone());
                response
            }
        }
    }
}

fn write_headers(headers: &mut HeaderMap, response: Response) {
    for header in response.headers {
        headers.insert(HeaderName::from_str(header.key.as_str()).unwrap(), HeaderValue::from_str(header.value.as_str()).unwrap());
    }
}

pub struct RouteMeta(pub String, pub HashMap<String, Route>);
pub fn config(cfg: &mut web::ServiceConfig) {
    let mut scope = web::scope("");
    let mut route_map = HashMap::new();
    let path_configs = extract_config_path();
    for handler in generate_handlers(path_configs.0.clone()).into_iter() {
        scope = scope.route(handler.1.as_str(), handler.2);
        route_map.insert(format!("{}-{}", handler.0.clone(), format!("/{}", handler.1.clone())), handler.3.clone());
    }
    scope = scope.app_data(Data::new(RouteMeta(path_configs.1.clone(), route_map)));
    cfg.service(scope);
}