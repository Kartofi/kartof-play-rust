use std::{ fs::{ self, copy, File }, io::{ Read, Write }, path::Path, thread };

use crate::SETTINGS;

use super::{ http, types::{ CacheResult, Image } };

pub fn save_image(id: String, url: String) -> CacheResult {
    if image_exits(&id) {
        return CacheResult::new("Image already exists!", true);
    }
    thread::spawn(move || {
        let stream = http::get_stream(&url);
        if stream.is_none() {
            return;
        }

        write_image(&id, stream.unwrap());
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
    if !Path::new(&SETTINGS.IMAGES_PATH).exists() {
        fs::create_dir(&SETTINGS.IMAGES_PATH).unwrap();
    }
}

fn image_exits(id: &str) -> bool {
    Path::new(&(SETTINGS.IMAGES_PATH.to_owned() + "/" + id + ".jpg")).exists()
}
fn get_image_local(id: &str) -> Image {
    let path_str = SETTINGS.IMAGES_PATH.to_owned() + "/" + id + ".jpg";
    let path = Path::new(&path_str);
    let data = fs::read(path).unwrap_or_default();

    Image::new(id, data)
}
fn write_image(id: &str, mut stream: impl Read) {
    let path_str = SETTINGS.IMAGES_PATH.to_owned() + "/" + id + ".jpg";
    let path = Path::new(&path_str);

    let mut file = File::create(path).unwrap();

    let mut buffer = [0; 1024];

    while stream.read(&mut buffer).is_ok() {
        file.write_all(&buffer).unwrap();
    }
}
