use ramhorns::{Content, Template};

use crate::errors::ApiError;
use std::env;

#[derive(Content)]
struct DashboardLogin<'a> {
    title: &'a str,
}

pub fn dashboard_login() -> Result<String, ApiError> {
    let tpl = Template::from_file(format!(
        "{}/dashboard_login.html",
        env::var("TEMPLATES_PATH")?
    ))?;

    let content = DashboardLogin {
        title: &format!("{} Dashboard", env::var("PLATFORM_NAME")?),
    };

    Ok(tpl.render(&content))
}
