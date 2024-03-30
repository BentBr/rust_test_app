mod auth;
mod to_do;
mod users;

use actix_web::web::ServiceConfig;
use auth::auth_views_factory;
use to_do::to_do_views_factory;
use users::user_views_factory;

pub fn views_factory(app: &mut ServiceConfig) {
    auth_views_factory(app);
    to_do_views_factory(app);
    user_views_factory(app);
}
