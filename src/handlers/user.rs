use actix_web::{cookie::Cookie, web, HttpMessage, HttpRequest, HttpResponse};
use validator::Validate;

use time::{Duration, OffsetDateTime};

use crate::db;
use crate::errors::ApiError;
use crate::mails as mail;
use actix::Addr;

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonBrancaToken {
    id: i32,
    token: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateUser {
    #[validate(email)]
    email: String,
    #[validate(length(min = 1))]
    username: String,
    #[validate(length(min = 5))]
    password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct AuthUser {
    #[validate(email)]
    email: String,
    #[validate(length(min = 5))]
    password: String,
}
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ChangePassword {
    #[validate(length(min = 5))]
    old_password: String,
    #[validate(length(min = 5))]
    new_password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Mail {
    #[validate(email)]
    email: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ResetPassword {
    #[validate(email)]
    email: String,
    token: String,
    password: String,
}

pub fn extract_json_token(req: HttpRequest) -> Result<JsonBrancaToken, ApiError> {
    let c = req
        .cookie("BrancaToken")
        .map(|cookie| cookie.value().to_string())
        .ok_or(ApiError::InternalError("MissingToken".to_owned()))?;
    let j: JsonBrancaToken = serde_json::from_str(&c)?;
    Ok(j)
}

pub async fn register(
    pool: web::Data<db::DbPool>,
    postman: web::Data<Addr<mail::Postman>>,
    input: web::Json<CreateUser>,
) -> Result<HttpResponse, ApiError> {
    input.validate()?;
    let db = pool.get()?;

    db::user::register(
        false,
        &input.0.username,
        &input.0.password,
        &input.0.email,
        &db,
    )?;

    mail::post_email(
        mail::user::create_register_email(input.0.email.as_ref(), input.0.username.as_ref())?,
        postman.get_ref(),
    )?;

    Ok(HttpResponse::Ok().finish())
}

pub async fn update(
    pool: web::Data<db::DbPool>,
    input: web::Json<CreateUser>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    input.validate()?;
    let db = pool.get()?;
    let j = extract_json_token(req)?;

    db::user::update(
        &j.id,
        &input.0.username,
        &input.0.password,
        &input.0.email,
        &db,
    )?;

    Ok(HttpResponse::Ok().finish())
}

pub async fn delete(
    pool: web::Data<db::DbPool>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let db = pool.get()?;
    let j = extract_json_token(req)?;
    db::user::delete(&j.id, &db)?;
    Ok(HttpResponse::Ok().finish())
}

pub async fn login(
    pool: web::Data<db::DbPool>,
    input: web::Json<AuthUser>,
) -> Result<HttpResponse, ApiError> {
    input.validate()?;
    let db = pool.get()?;

    let result = db::user::auth(&input.0.email, &input.0.password, &db)?;
    let c = Cookie::build("BrancaToken", result.to_owned())
        //.domain("www.rust-lang.org")
        .path("/")
        //.secure(true)
        .http_only(true)
        .finish();

    Ok(HttpResponse::Ok().cookie(c).finish())
}

pub async fn logout() -> Result<HttpResponse, ApiError> {
    let c = Cookie::build("BrancaToken", "")
        //.domain("www.rust-lang.org")
        .path("/")
        //.secure(true)
        .http_only(true)
        .max_age(Duration::zero())
        .expires(OffsetDateTime::now_utc() - Duration::days(365))
        .finish();
    Ok(HttpResponse::Ok().cookie(c).finish())
}

pub async fn get(pool: web::Data<db::DbPool>, req: HttpRequest) -> Result<HttpResponse, ApiError> {
    let db = pool.get()?;
    let j = extract_json_token(req)?;
    let user = db::user::get_user_by_id(&j.id, &db)?;
    Ok(HttpResponse::Ok().json(user))
}

pub async fn forgot_password(
    pool: web::Data<db::DbPool>,
    postman: web::Data<Addr<mail::Postman>>,
    input: web::Json<Mail>,
) -> Result<HttpResponse, ApiError> {
    input.validate()?;
    let db = pool.get()?;

    let user = db::user::get_user_by_email(&input.email, &db)?;
    let token = db::user::generate_reset_token(&input.email, &db)?;
    let mail = mail::user::create_reset_token_email(&input.email, &user.username, &token)?;
    mail::post_email(mail, postman.get_ref())?;

    Ok(HttpResponse::Ok().finish())
}

pub async fn reset_password(
    pool: web::Data<db::DbPool>,
    postman: web::Data<Addr<mail::Postman>>,
    input: web::Json<ResetPassword>,
) -> Result<HttpResponse, ApiError> {
    input.validate()?;
    let db = pool.get()?;

    let user = db::user::get_user_by_email(&input.email, &db)?;
    db::user::verify_reset_token(&input.email, &input.token, &db)?;
    db::user::change_password(&input.email, &input.password, &db)?;
    let mail = mail::user::create_password_changed_success_email(&input.email, &user.username)?;
    mail::post_email(mail, postman.get_ref())?;

    Ok(HttpResponse::Ok().finish())
}

pub async fn change_password(
    pool: web::Data<db::DbPool>,
    postman: web::Data<Addr<mail::Postman>>,
    input: web::Json<ChangePassword>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    input.validate()?;
    let db = pool.get()?;

    let j = extract_json_token(req)?;
    let user = db::user::get_user_by_id(&j.id, &db)?;
    db::user::auth(&user.email, &input.old_password, &db)?;
    db::user::change_password(&user.email, &input.new_password, &db)?;
    let mail = mail::user::create_password_changed_success_email(&user.email, &user.username)?;
    mail::post_email(mail, postman.get_ref())?;

    Ok(HttpResponse::Ok().finish())
}
