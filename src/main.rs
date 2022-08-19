use std::collections::HashMap;

use rocket::{fs::FileServer, *};
use rocket_dyn_templates::Template;

mod settings;
use settings::Settings;

mod fairings;

mod requests;

#[get("/")]
async fn index() -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render("default", &context)
}

#[launch]
fn rocket() -> _ {
    let settings = Settings::init();
    let mut routes: Vec<Route> = Vec::new();

    routes.append(&mut routes![index]);

    routes.append(&mut routes![
        requests::signup::signup,
        requests::login::login,
        requests::login::login_request,
        requests::signup::signup_request,
        requests::delete::delete,
        requests::delete::delete_request,
        requests::account::account,
    ]);

    rocket::build()
        .mount("/", routes)
        .mount("/", FileServer::from("public/"))
        .manage(settings)
        .attach(Template::fairing())
}
