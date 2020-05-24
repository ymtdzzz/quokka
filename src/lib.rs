use anyhow::Error;
use image::{open, DynamicImage, GenericImageView};
use std::fs;
use std::path::Path;
use headless_chrome::{Browser, protocol::page::*};
use headless_chrome::protocol::target::methods::CreateTarget;
use serde::{Serialize, Deserialize};
use url::Url;
use lcs_image_diff::compare;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Config {
    images: String,
    watch: String,
    url: String,
    settings: Vec<IndivisualSetting>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct IndivisualSetting {
    name: String,
    path: String,
}

impl Config {
    fn new(path: Option<&str>) -> Result<Config> {
        let p: &str;
        match path {
            Some(str) => p = str,
            None => p = ".quokkaconfig.json",
        }
        let file = fs::File::open(p)?;
        let reader = std::io::BufReader::new(file);
        let config = serde_json::from_reader(reader)?;
        Ok(config)
    }
}

impl IndivisualSetting {
    fn new(name: &str, path: &str) -> IndivisualSetting {
        IndivisualSetting {
            name: name.to_string(),
            path: path.to_string(),
        }
    }
}

pub fn get_diffs(config: &Config) -> Result<()> {
    let url = Url::parse(&config.url)?;
    for setting in &config.settings {
        let mut camp = image::open(Path::new(&config.images).join(&setting.name))?;
        let screenshot = take_screenshot(camp.width() as i32, camp.height() as i32, url.join(&setting.path)?.as_str());
        if let Ok(actual) = screenshot {
            let mut actual = image::load_from_memory(&actual)?;
            let diff = compare(&mut camp, &mut actual, 100.0 / 256.0)?;
            diff.save("aaaaa.png")?;
        }
    }
    Ok(())
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
    fn test_take_screenshot() {
        let image = take_screenshot(1920, 1080, "https://google.com");
        assert_eq!(true, image.is_ok());
        let image = take_screenshot(1920, 1080, "invalidurl");
        assert_eq!(true, image.is_err());
    }

    #[test]
    fn test_new_indivisual_setting() {
        let expected = IndivisualSetting {
            name: "index_pc.png".to_string(),
            path: "/".to_string(),
        };
        assert_eq!(expected, IndivisualSetting::new("index_pc.png", "/"));
    }

    #[test]
    fn test_new_config() {
        let expected = Config {
            images: "design_camp".to_string(),
            watch: "src".to_string(),
            url: "127.0.0.1:9001".to_string(),
            settings: vec![
                IndivisualSetting::new("index_pc.png", "/"),
                IndivisualSetting::new("index_sp.png", "/"),
                IndivisualSetting::new("posts_pc.png", "/posts"),
                IndivisualSetting::new("posts_sp.png", "/posts"),
            ]
        };
        let actual = Config::new(Some("test/testconfig.json"));
        assert_eq!(true, actual.is_ok());
        assert_eq!(expected, actual.unwrap());
        assert_eq!(true, Config::new(None).is_err());
    }

    #[test]
    fn test_get_diffs() {
        let config = Config::new(Some("test/testconfig2.json")).unwrap();
        get_diffs(&config);
    }
}
