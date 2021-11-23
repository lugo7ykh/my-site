use std::error::Error;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};

async fn handle_conn(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::new("hello".into())),
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

#[tokio::main]
pub async fn run() -> Result<(), Box<dyn Error + Send + Sync>> {
    let addr = ([127, 0, 0, 1], 4747).into();
    let service = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(handle_conn)) });
    let server = Server::bind(&addr).serve(service);

    println!("The web interface is available at http://{}", addr);
    server.await?;

    Ok(())
}
