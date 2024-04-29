use serde::{Deserialize, Serialize};
use warp::Filter;

use rust_embed::RustEmbed;

#[derive(Debug, Serialize, Deserialize)]
struct Query {
    name: String
}

#[derive(RustEmbed)]
#[folder = "web/build"]
struct Static;

#[tokio::main]
async fn main() {

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    // GET /myapp/hello?name=jdoe => 200 OK with body "Hello, jdoe!"
    let hello = warp::path!("myapp" / "hello")
        .and(warp::query::<Query>())
        .map(|q: Query| {
            log::debug!("/myapp/hello {:?}", &q);
            format!("Hello, {}!", q.name)
        });

    let static_files = warp::path("myapp")
        .and(warp::get())
        .and(warp_embed::embed(&Static))
        .boxed();

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST"])
        .allow_headers(vec!["Content-Type"]);

    log::info!("Serving myapp on http://localhost:3030/myapp");
    warp::serve(hello.or(static_files).with(cors))
        .run(([127, 0, 0, 1], 3030))
        .await;
}