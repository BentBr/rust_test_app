mod login;
mod logout;

use crate::views::handlers::json_handler::json_error_handler;
use crate::views::handlers::not_found_handler::not_found;
use actix_web::web::{get, route, scope, JsonConfig, ServiceConfig};

pub fn auth_views_factory(app: &mut ServiceConfig) {
    app.service(
        scope("v1/auth")
            .route("login", get().to(login::login))
            .route("logout", get().to(logout::logout))
            .default_service(route().to(not_found))
            .app_data(JsonConfig::default().error_handler(json_error_handler)),
    );
}
