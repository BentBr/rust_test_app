mod auth;
pub mod handlers;
mod to_do;
mod users;

use crate::views::handlers::not_found_handler::not_found;
use actix_web::web::{route, scope, ServiceConfig};
use auth::auth_views_factory;
use to_do::to_do_views_factory;
use users::user_views_factory;

pub fn views_factory(app: &mut ServiceConfig) {
    auth_views_factory(app);
    to_do_views_factory(app);
    user_views_factory(app);
    default_factory(app)
}

pub fn default_factory(app: &mut ServiceConfig) {
    app.service(scope("").default_service(route().to(not_found)));
}
