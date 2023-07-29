use rocket::request::FromRequest;

#[macro_use]
extern crate rocket;

// use tempfile form????
struct WfScreenshot {
    // image object here
}

struct Passive {
    // image correlating to it maybe
    desc: String,

    // instead of string should be some URL object
    wiki_link: String,
}

// note: should return array of results of names maybe
// or objects giving more info on each of the passives found
#[post("/find-passives-by-screenshot")]
fn find_image<'r>(screenshot: &'r WfScreenshot) -> &'static str {
    "todo"
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, find_image])
}
