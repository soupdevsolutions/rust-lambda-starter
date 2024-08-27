use lambda_http::{run, service_fn, tracing, Body, Error, Request, Response};

async fn function_handler(_event: Request) -> Result<Response<Body>, Error> {
    let message = "Hello, world!".to_string();
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(message.into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .json()
        .with_max_level(tracing::Level::INFO)
        .with_current_span(false)
        .with_ansi(false)
        .without_time()
        .with_target(false)
        .init();

    run(service_fn(function_handler)).await
}
