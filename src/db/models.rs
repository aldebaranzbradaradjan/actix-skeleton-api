use super::schema::users;
use chrono::NaiveDateTime;

#[derive(Serialize, Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub is_admin: bool,
    pub username: String,
    #[serde(skip_serializing)]
    pub email: String,
    #[serde(skip_serializing)]
    pub token_key: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub reset_token: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub is_admin: &'a bool,
    pub username: &'a str,
    pub email: &'a str,
    pub token_key: &'a str,
    pub password_hash: &'a str,
    pub reset_token: &'a str,
}
