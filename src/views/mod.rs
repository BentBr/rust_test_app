mod auth;
mod to_do;
mod app;

use actix_web::web::ServiceConfig;
use auth::auth_views_factory;
use to_do::to_do_views_factory;
use crate::views::app::app_views_factory;

pub fn views_factory(app: &mut ServiceConfig) {
    auth_views_factory(app);
    to_do_views_factory(app);
    app_views_factory(app);
}
