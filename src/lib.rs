use anyhow::Error;
use image::{open, DynamicImage};
use std::fs;
use std::path::Path;

pub type Result<T> = std::result::Result<T, Error>;

pub fn get_images(path: &str) -> Result<Vec<DynamicImage>> {
    let paths = fs::read_dir(path)?;
    let mut images: Vec<DynamicImage> = vec![];
    for path in paths {
        if let Ok(entry) = path {
            if is_image(entry.path().to_str().unwrap()) {
                images.push(open(entry.path())?);
            }
        }
    }
    Ok(images)
}

fn is_image(path: &str) -> bool {
    let path = Path::new(path);
    if path.is_file() {
        let filename = path.file_name();
        if let Some(f) = filename {
            let f = f.to_str();
            if let Some(fname) = f {
                let strs: Vec<&str> = fname.split('.').collect();
                if strs.len() > 1 {
                    let last_str = strs.last();
                    if let Some(last) = last_str {
                        match last {
                            &"png" | &"jpeg" | &"jpg" => return true,
                            _ => return false,
                        }
                    }
                }
            }
        };
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_image_with_image() {
        assert_eq!(true, is_image("test/images/pngimage.png"));
        assert_eq!(true, is_image("test/images/jpegimage.jpeg"));
        assert_eq!(true, is_image("test/images/jpegimage.jpg"));
    }

    #[test]
    fn is_image_with_other() {
        assert_eq!(false, is_image("test/images/image.txt"));
        assert_eq!(false, is_image("test/images/directory"));
    }

    #[test]
    fn get_images_works() {
        let images = get_images("test/images");
        assert_eq!(3, images.unwrap().len());
    }
}
