use std::collections::HashMap;

use rocket::form::{Form, FromForm};
use rocket::response::Redirect;
use rocket::*;
use rocket_dyn_templates::Template;

#[derive(Debug, FromForm)]
pub struct AccountLogin<'v> {
    username: &'v str,
    password: &'v str,
    remember: &'v str,
}

#[post("/delete", data = "<form>")]
pub async fn delete_request<'r>(form: Form<AccountLogin<'_>>) -> Redirect {
    println!(
        "Username = {} | Password = {} | remember = {}",
        form.username, form.password, form.remember
    );
    Redirect::to("/")
}

#[get("/delete")]
pub async fn delete() -> Template {
    Template::render("delete", &HashMap::<String, String>::new())
}
