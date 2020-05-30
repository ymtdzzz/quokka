extern crate quokka;

use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc;
use std::thread;
use warp::{http::Uri, Filter};
use std::str::FromStr;
use quokka::*;

// GRCOV_EXCL_START
#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::new(None)?;
    // let (tx, rx) = mpsc::channel();
    // let mut watcher: RecommendedWatcher = Watcher::new_immediate(move |res| {
    //     match res {
    //         Ok(event) => tx.send(format!("event: {:?}", event)).unwrap(),
    //         Err(e) => tx.send(format!("watch error: {:?}", e)).unwrap(),
    //     }
    // })?;
    // watcher.configure(notify::Config::OngoingEvents(Some(
    //     std::time::Duration::from_millis(500),
    // )));
    // watcher.watch(&config.watch, RecursiveMode::Recursive)?;

    let diffs = get_diffs(&config)?;
    // fn reply(images: &Vec<image::DynamicImage>) -> impl warp::reply::Reply {
    //     let body = generate_html(images.to_vec());
    //     warp::reply::html(body)
    // }
    let response = warp::path::end()
        .map(move || {
            reply(&diffs)
    });

    warp::serve(response).run(([127, 0, 0, 1], 3030)).await;
    // loop {
    //     let event = rx.recv().unwrap();
    //     println!("Got: {:?}", event);
    // }

    Ok(())
}
// GRCOV_EXCL_STOP
