use actix_web::web::{delete, get, patch, post, scope, ServiceConfig};

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
            .route("password/{uuid}", patch().to(edit::password)),
    );
}
