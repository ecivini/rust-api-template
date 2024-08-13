pub mod health;

use warp::{
    filters::BoxedFilter,
    reply::Reply,
};

/// Returns all handlers
pub fn get_all_handlers() -> BoxedFilter<(impl Reply,)> {
    let health_router = health::build_health_endpoints();

    health_router
}
