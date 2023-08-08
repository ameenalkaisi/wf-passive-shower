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

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![upload_image])
}

#[cfg(test)]
mod tests {
    use std::{
        env,
        io::{self, Read, Write},
        path::PathBuf,
    };

    use super::rocket;
    use image::RgbImage;
    use rocket::{
        http::{Header, Status},
        local::blocking::Client,
    };
    use std::fs::File;

    // https://stackoverflow.com/questions/66921683/rocket-testing-multipart-data-form-fails-with-422-unprocessable-entity
    // used above for image uploading functionality
    fn create_image(file_name: &str) -> PathBuf {
        let path_buf = env::temp_dir().join(file_name);

        let imgbuf: RgbImage = image::ImageBuffer::new(256, 256);
        imgbuf.save(path_buf.as_path()).unwrap();

        path_buf
    }

    // creates image data section, to be used in multipart send
    fn image_data(boundary: &str, image_path: PathBuf) -> io::Result<Vec<u8>> {
        // https://stackoverflow.com/questions/51397872/how-to-post-an-image-using-multipart-form-data-with-hyper
        // https://golangbyexample.com/multipart-form-data-content-type-golang/
        let mut data = Vec::new();

        // start
        write!(data, "--{}\r\n", boundary)?;

        // image data
        write!(
            data,
            "Content-Disposition: form-data; name=\"image\"; filename=\"image.jpg\"\r\n"
        )?;
        write!(data, "Content-Type: image/jpeg\r\n")?;
        write!(data, "\r\n")?;

        // let path_buf = create_image("image.jpg");
        let path_buf = image_path;

        let mut f = File::open(path_buf.as_path())?;
        f.read_to_end(&mut data)?;

        write!(data, "\r\n")?;
        write!(data, "--{}\r\n", boundary)?;

        Ok(data)
    }

    #[test]
    fn upload_works_with_random_image() -> anyhow::Result<()> {
        const BOUNDARY: &str = "--------------------------------XYZ";

        let client = Client::tracked(rocket()).unwrap();

        let content_type = Header::new(
            "Content-Type",
            format!("multipart/form-data; boundary={}", BOUNDARY),
        );

        let response = client
            .post(uri!(super::upload_image))
            .header(content_type)
            .body(image_data(BOUNDARY, create_image("image.jpg")).unwrap())
            .dispatch();

        assert_eq!(response.status(), Status::Ok);

        Ok(())
    }

    #[test]
    fn file_is_saved() -> anyhow::Result<()> {
        const BOUNDARY: &str = "--------------------------------XYZ";

        let client = Client::tracked(rocket()).unwrap();

        let content_type = Header::new(
            "Content-Type",
            format!("multipart/form-data; boundary={}", BOUNDARY),
        );

        let response = client
            .post(uri!(super::upload_image))
            .header(content_type)
            .body(image_data(BOUNDARY, PathBuf::from("test/first.png")).unwrap())
            .dispatch();

        assert_eq!(response.status(), Status::Ok);

        Ok(())
    }
}
