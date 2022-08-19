use rocket::http::{Cookie, Status};
use rocket::request::{self, FromRequest, Outcome, Request};

use misato::{
    models::account_model,
    routes::admin::{account, account::Routes},
};

use crate::settings::Settings;

pub struct AccountToken {
    pub account: account_model::Account,
}

#[derive(Debug)]
pub enum AccountTokenError {
    Missing,
    Invalid,
}

async fn get_profile(token: String, settings: &Settings) -> Result<account_model::Account, Status> {
    let request = misato::request::Request::new(None, Some(settings.misato_api_token.to_string()));

    let token_infos = account::CheckToken(account_model::AccountToken {
        token: token.clone(),
    })
    .execute(&request)
    .await;

    if token_infos.is_err() {
        return Err(Status::InternalServerError);
    }

    let token_infos = token_infos.unwrap();

    if token_infos.is_err() {
        return Err(Status::BadRequest);
    }

    let account = account::Profile(account_model::AccountUuid {
        uuid: token_infos.unwrap().uuid,
    })
    .execute(&request)
    .await;

    if account.is_err() {
        return Err(Status::InternalServerError);
    }

    Ok(account.unwrap().unwrap())
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AccountToken {
    type Error = AccountTokenError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<AccountToken, Self::Error> {
        let keys: Option<&Cookie> = request.cookies().get("Account-Token");
        match keys {
            Some(cookie) => {
                let settings = request.rocket().state::<Settings>().unwrap();

                let response = get_profile(cookie.value().to_string(), settings).await;

                match response {
                    Ok(account) => {
                        return Outcome::Success(AccountToken { account });
                    }
                    Err(err) => {
                        request.cookies().remove(Cookie::named("Account-Token"));
                        return Outcome::Failure((err, AccountTokenError::Invalid));
                    }
                }
            }
            None => return Outcome::Failure((Status::BadRequest, AccountTokenError::Missing)),
        }
    }
}
