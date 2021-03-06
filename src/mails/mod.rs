pub mod user;

use actix::prelude::*;
use actix::Addr;

extern crate lettre;

use lettre::{smtp::authentication::Credentials, SmtpClient, Transport};
use lettre_email::EmailBuilder;

use log::info;
use std::env;

use crate::errors::ApiError;

#[derive(Message)]
#[rtype(result = "Result<bool, std::io::Error>")]
pub struct SendableEmail {
    to: String,
    title: String,
    content: String,
}

pub fn send_mail(mail: SendableEmail) -> Result<(), ApiError> {
    let email = EmailBuilder::new()
        .to(mail.to)
        .from(env::var("SMTP_CREDENTIAL")?)
        .subject(mail.title)
        .html(mail.content)
        .build()?;

    let mut mailer = SmtpClient::new_simple(&env::var("SMTP_URL")?)?
        .credentials(Credentials::new(
            env::var("SMTP_CREDENTIAL")?,
            env::var("SMTP_PASSWORD")?,
        ))
        .transport();

    mailer.send(email.into())?;
    Ok(())
}

pub struct Postman;

impl Actor for Postman {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        info!("Starting Postman Actor");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        info!(">Shut down Postman Actor");
    }
}

impl Handler<SendableEmail> for Postman {
    type Result = Result<bool, std::io::Error>;

    fn handle(&mut self, email: SendableEmail, _ctx: &mut Context<Self>) -> Self::Result {
        info!("SendableEmail received, processing...");
        match send_mail(email) {
            Ok(()) => Ok(true),
            Err(err) => {
                info!("Error on send email : {}", err);
                Ok(false)
            }
        }
    }
}

pub fn post_email(email: SendableEmail, actor: &Addr<Postman>) -> Result<(), ApiError> {
    match actor.try_send(email) {
        Ok(()) => Ok(()),
        Err(_) => Err(ApiError::InternalError(
            "The postman can't send email".into(),
        )),
    }
}
