use std::{fs, path::Path, thread};

use crate::IMAGES_PATH;

use super::{
    http,
    types::{CacheResult, Image},
};

pub fn save_image(id: String, url: String) -> CacheResult {

    thread::spawn(move || {
        let data = http::get_bytes(&url);
        if data.is_none() {
            return;
        }

        let image = Image::new(&id, data.unwrap());

        write_image(image);
    });
    CacheResult::new("Saved image!", false)
}

pub fn get_image(id: &str) -> Option<Image> {
    if image_exits(id) == false {
        return None;
    }
    Some(get_image_local(id))
}

pub fn setup() {
    if !Path::new(IMAGES_PATH).exists() {
        fs::create_dir(IMAGES_PATH).unwrap();
    }
}

fn image_exits(id: &str) -> bool {
    Path::new(&(IMAGES_PATH.to_owned() + "/" + id + ".jpg")).exists()
}
fn get_image_local(id: &str) -> Image {
    let path_str = IMAGES_PATH.to_owned() + "/" + id + ".jpg";
    let path = Path::new(&path_str);
    let data = fs::read(path).unwrap_or_default();

    Image::new(id, data)
}
fn write_image(image: Image) {
    let path_str = IMAGES_PATH.to_owned() + "/" + &image.id + ".jpg";
    let path = Path::new(&path_str);

    fs::write(path, image.data).unwrap();
}
