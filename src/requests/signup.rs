use std::collections::HashMap;

use misato::models::account_model::AccountCredentials;
use misato::request::Request;
use rocket::form::{Form, FromForm};
use rocket::http::{Cookie, CookieJar, Status};
use rocket::response::Redirect;
use rocket::*;
use rocket_dyn_templates::Template;

use misato::request::Response::*;
use misato::routes::admin::account::{self, Routes};

use crate::settings::Settings;

#[derive(Debug, FromForm)]
pub struct AccountSignup<'v> {
    username: &'v str,
    password: &'v str,
}

#[post("/signup", data = "<form>")]
pub async fn signup_request<'r>(
    form: Form<AccountSignup<'_>>,
    settings: &State<Settings>,
    jar: &CookieJar<'_>,
) -> Result<Redirect, Status> {
    let credentials: AccountCredentials = AccountCredentials {
        username: form.username.to_string(),
        password: form.password.to_string(),
    };
    let request = Request::new(None, Some(settings.misato_api_token.to_string()));

    let response = account::Signup(credentials).execute(&request).await;

    if response.is_err() {
        return Err(Status::InternalServerError);
    }

    let response = response.unwrap();

    match response {
        Success(success) => {
            jar.add(Cookie::new("X-Misato-Token", success.token));
            Ok(Redirect::to("/profile"))
        }
        Error(_) => Ok(Redirect::to("/signup")),
    }
}

#[get("/signup")]
pub async fn signup() -> Template {
    Template::render("signup", &HashMap::<String, String>::new())
}
