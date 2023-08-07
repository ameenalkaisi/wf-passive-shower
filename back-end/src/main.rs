use rocket::{form::Form, fs::TempFile};

#[macro_use]
extern crate rocket;

// use tempfile form????
// note: should accept ANY image type
// might depend on the tool used to do the pattern matching instead of at the data layer
#[derive(FromForm)]
struct IncomingWfScreenshot<'r> {
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
async fn upload_image(mut form: Form<IncomingWfScreenshot<'_>>) -> std::io::Result<()> {
    form.img.persist_to("./file.png").await?;
    Ok(())
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, upload_image])
}

#[cfg(test)]
mod tests {
    use super::rocket;
    use rocket::{
        http::{ContentType, Status},
        local::blocking::Client,
    };

    #[test]
    fn file_is_saved() -> anyhow::Result<()> {
        let img = image::io::Reader::open("myimage.png")?.decode().unwrap();
        // let reader = BufReader::open(File::open("test/first.png").unwrap()).;

        let mut client = Client::tracked(rocket()).unwrap();

        let mut req = client
            .post(uri!(super::upload_image))
            .header(ContentType::FormData)
            // up to here correct
            //
            // need to do special things here
            //
            // something to do with boundaries not sure
            .body(img);

        let response = req.dispatch();
        assert_eq!(response.status(), Status::Ok);

        Ok(())
    }
}
