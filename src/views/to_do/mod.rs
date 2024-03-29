mod create;
mod delete;
mod edit;
mod get;
mod get_one;

use actix_web::web::{delete, get, patch, post, scope, ServiceConfig};

pub fn to_do_views_factory(app: &mut ServiceConfig) {
    app.service(
        scope("v1/task")
            .route("create", post().to(create::create))
            .route("get", get().to(get::get))
            .route("get/{uuid}", get().to(get_one::get_one))
            .route("edit/{uuid}", patch().to(edit::edit))
            .route("delete/{uuid}", delete().to(delete::delete)),
    );
}
