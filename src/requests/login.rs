use std::collections::HashMap;

use rocket::form::{Form, FromForm};
use rocket::http::CookieJar;
use rocket::response::Redirect;
use rocket::*;
use rocket_dyn_templates::Template;

use crate::fairings::account_authentication::AccountToken;
use crate::settings::Settings;

#[derive(Debug, FromForm)]
pub struct AccountLogin<'v> {
    username: &'v str,
    password: &'v str,
    remember: &'v str,
}

#[post("/login", data = "<form>")]
pub async fn login_request<'r>(
    form: Form<AccountLogin<'_>>,
    settings: &State<Settings>,
    jar: &CookieJar<'_>,
) -> Redirect {
    Redirect::to("/profile")
}

#[get("/login")]
pub async fn login(token: Option<AccountToken>) -> Result<Template, Redirect> {
    if token.is_some() {
        return Err(Redirect::to("/profile"));
    }

    Ok(Template::render("login", &HashMap::<String, String>::new()))
}
