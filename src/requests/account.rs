use rocket::*;
use rocket_dyn_templates::{context, Template};

use crate::fairings::account_authentication::AccountToken;

#[get("/profile")]
pub async fn account(token: Option<AccountToken>) -> Template {
    if token.is_none() {
        return Template::render("login", context! {});
    }

    let account = token.unwrap().account;

    Template::render(
        "account",
        context! {
            uuid: account.uuid,
            username: account.username,
        },
    )
}
