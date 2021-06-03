 
#[macro_use]
extern crate diesel;
extern crate dotenv;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate chrono;

use dotenv::dotenv;
use structopt::StructOpt;

mod middlewares;
mod handlers;
mod db;
mod templates;
mod mails;
mod errors;

use crate::{
  db as database,
  errors::*,
  mails as mail
};

#[derive(StructOpt, Debug)]
#[structopt(about = "a tool to manage db and api")]
enum Cli {

    CreateUser {
      #[structopt(short, help = "The new user will be admin")]
      admin: bool,
      #[structopt(short, long)]
      email: String,
      #[structopt(short, long)]
      password: String,
      #[structopt(short, long)]
      username: Option<String>,
    },

    DeleteUser {
      email: String,
    },

    SendMail {
      to: String,
      title: String,
      content: String,
    },

    SendRegisterMail {
      to: String,
      username: String,
    },

    SendResetMail {
      to: String,
      username: String,
      token: String,
    },

}

fn main() -> Result<(), ApiError> {
  let args = Cli::from_args();
  dotenv().ok();
  env_logger::init();

  let pool = database::init_pool().expect("Failed to create pool");

  match args {

    // ./artisan create-user -a -e florian.zebidi@gmx.fr -p eindoven -u Florian
    Cli::CreateUser {admin, email, password, username} => {

      let conn = pool.get() ? ;

      let name = match username {
        Some(x) => x,
        None    => "admin".to_string(),
      };

      let result = database::user::register(
        admin,
        name.as_ref(),
        password.as_ref(),
        email.as_ref(),
        &conn
      ) ? ;

      println!("Succefully created new user, id : {}", result);
      Ok(())
    },


    Cli::DeleteUser {email} => {
      println!("delete");
      Ok(())
    },


    // ./artisan send-mail "e.k.florian@gmail.com" "<h1>Je t'ai écris un mail en HTML avec du Rust</h1>Accessoirement c'est trop bien."
    Cli::SendMail {to, title, content} => {

      mail::send_mail(&to, &title, &content) ? ;
      Ok(())

    },

    Cli::SendRegisterMail {to, username} => {

      mail::user::send_register_email(&to, &username) ? ;
      Ok(())

    },

    Cli::SendResetMail {to, username, token} => {

      mail::user::send_reset_token_email(&to, &username, &token) ? ;
      Ok(())

    },


  }
}