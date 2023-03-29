mod hn;

use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {
    url: String,
}

#[derive(Serialize)]
struct Response {
    ok: bool,
    url: Option<String>,
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    let res = hn::fetch_hn_discussion_url(&event.payload.url).await;

    Ok(match res {
        Ok(url) => Response { ok: true, url: url },
        _ => Response {
            ok: false,
            url: None,
        },
    })
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
