use warp::Filter;

/// Health endpoint
/// Returns 200 OK
async fn get_health() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply())
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Create endpoints

pub fn build_health_endpoints() -> warp::filters::BoxedFilter<(impl warp::Reply,)> {
    let get_health_ep = warp::get()
        .and(warp::path("health"))
        .and(warp::path::end())
        .and_then(get_health);

    let routes = get_health_ep;

    routes.boxed()
}
