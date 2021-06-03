use actix_web::HttpResponse;

use crate::errors::ApiError;
use crate::templates::dashboard as tp;

pub async fn dashboard_login() -> Result<HttpResponse, ApiError> {
    let html = tp::dashboard_login()?;
    Ok(HttpResponse::Ok().body(html))
}
