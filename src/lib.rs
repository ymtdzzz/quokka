use anyhow::Error;
use headless_chrome::protocol::target::methods::CreateTarget;
use headless_chrome::{protocol::page::*, Browser};
use image::{open, DynamicImage, GenericImageView};
use image_diff::diff;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::str::FromStr;
use std::sync::mpsc;
use std::thread;
use url::Url;
use warp::{http::Uri, Filter};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Config {
    images: String,
    pub watch: String,
    url: String,
    settings: Vec<IndivisualSetting>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct IndivisualSetting {
    name: String,
    path: String,
}

impl Config {
    pub fn new(path: Option<&str>) -> Result<Config> {
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

// main process triggered by detecting file change
pub fn run(config: &Config) -> Result<()> {
    let diffs = get_diffs(&config)?;
    let html = generate_html(diffs);
    // let response = warp::path::end()
    //     .map(|| warp::reply::html(html.as_str()));

    // TODO: generate HTML response (images are displayed with byte array(https://stackoverflow.com/questions/20756042/how-to-display-an-image-stored-as-byte-array-in-html-javascript))
    // TODO: return Response
    // TODO: force reload
    Ok(())
}

pub fn generate_html(diffs: Vec<DynamicImage>) -> String {
    let html = r#"
<html>
    <head>
        <title>HTML with warp!</title>
    </head>
    <body>
        {}
    </body>
</html>
"#;
    let body = generate_body(diffs);

    html.replace("{}", &body)
}

fn generate_body(diffs: Vec<DynamicImage>) -> String {
    let mut i = 0;
    let image_tags: Vec<_> = diffs
        .iter()
        .map(|d| {
            i += 1;
            let mut buffer = Vec::new();
            let result = d.write_to(&mut buffer, image::ImageOutputFormat::Png);
            if let Ok(_) = result {
                format!(
                    r#"
    <img src="data:image/png;base64,{}">
    "#,
                    base64::encode(buffer)
                )
            } else {
                "".to_string()
            }
        })
        .collect();
    image_tags.join("\n").to_owned()
}

pub fn get_diffs(config: &Config) -> Result<Vec<DynamicImage>> {
    let url = Url::parse(&config.url)?;
    let mut result: Vec<DynamicImage> = vec![];
    for setting in &config.settings {
        let camp = image::open(Path::new(&config.images).join(&setting.name))?;
        let screenshot = take_screenshot(
            camp.width() as i32,
            camp.height() as i32,
            url.join(&setting.path)?.as_str(),
        );
        if let Ok(actual) = screenshot {
            result.push(diff(&camp, &actual)?);
        }
    }
    Ok(result)
}

fn take_screenshot(
    width: i32,
    height: i32,
    url: &str,
) -> anyhow::Result<DynamicImage, failure::Error> {
    // TODO: change width and height depends on the display scale settings
    let browser = Browser::default()?;
    let tab = browser.new_tab_with_options(CreateTarget {
        url: url,
        width: Some(width / 2),
        height: Some(height / 2),
        browser_context_id: None,
        enable_begin_frame_control: None,
    })?;
    tab.wait_for_element("body")?;
    let screenshot = tab.capture_screenshot(ScreenshotFormat::PNG, None, true)?;

    let screenshot = image::load_from_memory(&screenshot)?;

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
            ],
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
        panic!();
    }
}
