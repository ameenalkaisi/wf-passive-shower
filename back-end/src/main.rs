use rocket::{form::Form, fs::TempFile};

#[macro_use]
extern crate rocket;

// use tempfile form????
// note: should accept ANY image type
// might depend on the tool used to do the pattern matching instead of at the data layer
#[derive(FromForm)]
struct WfScreenshot<'r> {
    // image object here
    #[field(name = "image")]
    img: TempFile<'r>,
}

struct Passive {
    // image correlating to it maybe
    desc: String,

    // instead of string should be some URL object
    wiki_link: String,
}

// note: should return array of results of names maybe
// or objects giving more info on each of the passives found
#[post("/find-passives-from-ss", data = "<form>")]
async fn upload_image(mut form: Form<WfScreenshot<'_>>) -> std::io::Result<()> {
    form.img.persist_to("/tmp/complete/file.png").await?;
    Ok(())
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    println!(env!("CARGO_MANIFEST_DIR"));
    rocket::build().mount("/", routes![index, upload_image])
}

#[cfg(test)]
mod tests {
    use super::rocket;
    use rocket::{
        http::{ContentType, Status},
        local::blocking::Client,
    };
    use std::{
        fs::File,
        io::{BufReader, Read},
    };

    #[test]
    fn file_is_saved() {
        let mut img: String = "";
        let reader = BufReader::new(File::open("test/first.png").unwrap());

        let mut client = Client::tracked(rocket()).unwrap();

        let mut req = client
            .post(uri!(super::upload_image))
            .header(ContentType::FormData)
            // up to here correct
            //
            // need to do special things here
            .body(img);

        let response = req.dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
