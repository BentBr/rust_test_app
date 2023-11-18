mod create;
mod get;
mod get_one;
mod edit;
mod delete;

use actix_web::web::{delete, get, post, scope, ServiceConfig};

pub fn to_do_views_factory(app: &mut ServiceConfig) {
    app.service(
        scope("v1/item")
            .route("create/{title}", post().to(create::create))
            .route("get", get().to(get::get))
            .route("get/{title}", get().to(get_one::get_one))
            .route("edit", post().to(edit::edit))
            .route("delete/{title}", delete().to(delete::delete)),
    );
}
