use crate::views::handlers::json_handler::json_error_handler;
use crate::views::handlers::not_found_handler::not_found;
use actix_web::web::{delete, get, patch, post, route, scope, JsonConfig, ServiceConfig};

mod create;
mod delete;
mod edit;
mod get;
mod get_one;

pub fn user_views_factory(app: &mut ServiceConfig) {
    app.service(
        scope("v1/user")
            .route("create", post().to(create::create))
            .route("get/{uuid}", get().to(get_one::get_one))
            .route("get", get().to(get::get))
            .route("edit/{uuid}", patch().to(edit::edit))
            .route("delete/{uuid}", delete().to(delete::delete))
            .route("password/{uuid}", patch().to(edit::password))
            .default_service(route().to(not_found))
            .app_data(JsonConfig::default().error_handler(json_error_handler)),
    );
}
