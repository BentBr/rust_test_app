use crate::json_serialization::response::response_item::ResponseItem;
use crate::json_serialization::response::response_status::ResponseStatus;
use actix_web::{HttpRequest, HttpResponse};
use uuid::Uuid;

pub fn parse_uuid_from_request(request: HttpRequest) -> Result<Uuid, HttpResponse> {
    let uuid_option = request.match_info().get("uuid");
    let uuid_string: &str;

    match uuid_option {
        Some(uuid) => {
            uuid_string = uuid;
        }
        None => {
            return Err(HttpResponse::BadRequest().json(ResponseItem::new(
                ResponseStatus::Error,
                "Uuid parsing error".to_string(),
                "Check your uuid for it's format. Must be uuid_v4".to_string(),
            )))
        }
    }

    let uuid_result = Uuid::parse_str(uuid_string);

    match uuid_result {
        Err(error) => Err(HttpResponse::BadRequest().json(ResponseItem::new(
            ResponseStatus::Error,
            "Uuid has an error".to_string(),
            error.to_string(),
        ))),
        Ok(valid_uuid) => Ok(valid_uuid),
    }
}

#[cfg(test)]
mod tests {
    use super::parse_uuid_from_request;
    use actix_web::body::to_bytes;
    use actix_web::web::{get, scope, ServiceConfig};
    use actix_web::{test, App, HttpResponse};
    use serde_json::Value;
    use uuid::Uuid;

    fn test_factory(app: &mut ServiceConfig) {
        app.service(scope("v1/test").route("get/{uuid}", get().to(test_handler)));
    }
    async fn test_handler() -> HttpResponse {
        HttpResponse::Ok().json("test")
    }

    #[actix_rt::test]
    async fn test_parse_uuid_from_request_valid() {
        let uuid = Uuid::new_v4().to_string();
        let mut app = test::init_service(App::new().configure(test_factory)).await;
        let request = test::TestRequest::with_uri(&format!("/v1/test/get/{}", uuid)).to_request();

        // We have to create the response in order to get the correct request needed
        let response = test::call_service(&mut app, request).await;

        assert_eq!(
            parse_uuid_from_request(response.request().clone())
                .unwrap()
                .to_string(),
            uuid.to_string()
        );
    }

    #[actix_rt::test]
    async fn test_parse_uuid_from_request_invalid() {
        let mut app = test::init_service(App::new().configure(test_factory)).await;
        let request = test::TestRequest::with_uri("/v1/test/get/no-uuid").to_request();

        // We have to create the response in order to get the correct request needed
        let response = test::call_service(&mut app, request).await;

        match parse_uuid_from_request(response.request().clone()) {
            Ok(_) => panic!("Test for parsing uuid from request fails!"),
            Err(error) => {
                let body_bytes = to_bytes(error.into_body()).await.unwrap();
                let body_json: Value = serde_json::from_slice(&body_bytes).unwrap();

                assert_eq!(body_json["message"], "Uuid has an error");
                assert_eq!(
                    body_json["data"],
                    "invalid character: expected an optional prefix of `urn:uuid:` followed by [0-9a-fA-F-], found `n` at 1"
                );
            }
        }
    }

    #[actix_rt::test]
    async fn test_parse_uuid_from_request_missing() {
        let mut app = test::init_service(App::new().configure(test_factory)).await;
        let request = test::TestRequest::with_uri("/v1/test/get/").to_request();

        // We have to create the response in order to get the correct request needed
        let response = test::call_service(&mut app, request).await;

        match parse_uuid_from_request(response.request().clone()) {
            Ok(_) => panic!("Test for parsing uuid from request fails!"),
            Err(error) => {
                let body_bytes = to_bytes(error.into_body()).await.unwrap();
                let body_json: Value = serde_json::from_slice(&body_bytes).unwrap();

                assert_eq!(body_json["message"], "Uuid parsing error");
                assert_eq!(
                    body_json["data"],
                    "Check your uuid for it's format. Must be uuid_v4"
                );
            }
        }
    }
}
