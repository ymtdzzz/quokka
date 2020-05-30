extern crate quokka;

use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use quokka::*;
use std::sync::mpsc;
use std::thread;
use warp::{http::Uri, Filter};

// GRCOV_EXCL_START
#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::new(None)?;
    let (tx, rx) = mpsc::channel();

    let mut watcher: RecommendedWatcher = Watcher::new_immediate(move |res| {
        match res {
            Ok(event) => tx.send(format!("event: {:?}", event)).unwrap(),
            Err(e) => tx.send(format!("watch error: {:?}", e)).unwrap(),
        }
    })?;

    // watcher.configure(notify::Config::OngoingEvents(Some(
    //     std::time::Duration::from_millis(500),
    // )));
    watcher.watch(&config.watch, RecursiveMode::Recursive)?;

    let body = r#"
<html>
    <head>
        <title>HTML with warp!</title>
    </head>
    <body>
        <h1>Response Test</h1>
    </body>
</html>
"#;

    let hello = warp::path::end()
        .map(move || warp::reply::html(body));

    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
    loop {
        let event = rx.recv().unwrap();
        println!("Got: {:?}", event);
    }

    Ok(())
}
// GRCOV_EXCL_STOP
