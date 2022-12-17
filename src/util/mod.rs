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

#[actix_web::test]
async fn decode_jwt_token_test() {
    let mut jwt = "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyLCJjaWQiOiIxMjM0IiwiY291bnRyeSI6IkZJIn0.1xkM--PUZj2PIEXfZ0EMHPF2vgujKHwOREoDzg1dmxE";
    let claim = decode_jwt(Some(format!("{}", jwt)));
    assert_eq!("1234-FI", claim.user())
}
