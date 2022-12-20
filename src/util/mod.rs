use std::path::PathBuf;
use std::str::FromStr;
use clap::{Arg, Command};
use crate::model::TokenClaim;

pub fn decode_jwt(bearer: Option<String>) -> TokenClaim {
    match bearer {
        None => {
            panic!("no jwt supplied");
        }
        Some(data) => {
            let mut jwt = "";
            for (index, v) in data.char_indices() {
                if index == 7 {
                    jwt = &data[7..];
                    break;
                }
            }
            let mut parts: Vec<&str> = jwt.split(".").collect();
            let mut parts_iter = parts.iter();
            parts_iter.next().unwrap();
            let raw_claims = parts_iter.next().unwrap();
            let raw = base64_url::decode(raw_claims).unwrap();
            let raw = String::from_utf8(raw).unwrap();
            serde_json::from_str(raw.as_str()).unwrap()
        }
    }
}

pub fn extract_config_path() -> (String, String) {
    let cfg = Command::new("Mockoon Rustify")
        .version("1.0.0")
        .author("Al-Ani, Mohammed")
        .about("API mocking")
        .arg(Arg::new("data").required(true).short('d').long("data"))
        .try_get_matches();

    match cfg {
        Ok(arg_match) => {
            let api_config_file = arg_match.get_one::<String>("data").unwrap();
            let path_buf = PathBuf::from(api_config_file);
            let path_buf = std::fs::canonicalize(&path_buf).unwrap();
            (path_buf.display().to_string(), path_buf.parent().unwrap().display().to_string())
        }
        Err(_) => {
            let api_config_file = "data/api-config.json";
            let path_buf = PathBuf::from(api_config_file);
            let path_buf = std::fs::canonicalize(&path_buf).unwrap();
            (path_buf.display().to_string(), path_buf.parent().unwrap().display().to_string())
        }
    }
}

#[actix_web::test]
async fn decode_jwt_token_test() {
    let mut jwt = "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyLCJjaWQiOiIxMjM0IiwiY291bnRyeSI6IkZJIn0.1xkM--PUZj2PIEXfZ0EMHPF2vgujKHwOREoDzg1dmxE";
    let claim = decode_jwt(Some(format!("{}", jwt)));
    assert_eq!("1234-FI", claim.user())
}

#[actix_web::test]
async fn extract_dir_from_file_path() {
    init_logger();
    // custom dir not root
    let file = "data/api-config.json";
    let buf = PathBuf::from(file);
    let buf = std::fs::canonicalize(&buf).unwrap();

    log::info!("{}", buf.display());
    log::info!("{}", buf.parent().unwrap().display());

    let response_file_path = "1234-FI/data.json";
    let file_path = buf.parent().unwrap().join(response_file_path);
    log::info!("file_path = {}", file_path.display());
    let json = std::fs::read_to_string(file_path).unwrap();
    log::info!("file_data = {}", json);

}


pub fn init_logger() {
    use chrono::Local;
    use env_logger::Builder;
    use log::LevelFilter;
    use std::io::Write;

    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    Builder::new()
        .format(|buf, record| {
            writeln!(buf,
                     "{} [{}] - {}",
                     Local::now().format("%Y-%m-%dT%H:%M:%S"),
                     record.level(),
                     record.args()
            )
        })
        .filter(None, LevelFilter::Debug)
        .init();
}
