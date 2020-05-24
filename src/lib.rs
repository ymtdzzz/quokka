use anyhow::Error;
use image::{open, DynamicImage};
use std::fs;
use std::path::Path;
use headless_chrome::{Browser, protocol::page::*};
use headless_chrome::protocol::target::methods::CreateTarget;

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

fn take_screenshot(width: i32, height: i32, url: &str) -> anyhow::Result<Vec<u8>, failure::Error> {
    let browser = Browser::default()?;
    let tab = browser.new_tab_with_options(CreateTarget {
        url: url,
        width: Some(width),
        height: Some(height),
        browser_context_id: None,
        enable_begin_frame_control: None,
    })?;
    tab.wait_for_element("body")?;
    let screenshot = tab.capture_screenshot(
        ScreenshotFormat::PNG,
        None,
        true
    )?;
   
    Ok(screenshot)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_image() {
        assert_eq!(true, is_image("test/images/pngimage.png"));
        assert_eq!(true, is_image("test/images/jpegimage.jpeg"));
        assert_eq!(true, is_image("test/images/jpegimage.jpg"));

        assert_eq!(false, is_image("test/images/image.txt"));
        assert_eq!(false, is_image("test/images/png"));
        assert_eq!(false, is_image("test/images/directory"));
    }

    #[test]
    fn test_get_images() {
        let images = get_images("test/images");
        assert_eq!(true, images.is_ok());
        assert_eq!(3, images.unwrap().len());
       
        let images = get_images("no/exist/path");
        assert_eq!(true, images.is_err());
    }


    #[test]
    fn test_take_screenshot() {
        let image = take_screenshot(1920, 1080, "https://google.com");
        assert_eq!(true, image.is_ok());
    }
}
