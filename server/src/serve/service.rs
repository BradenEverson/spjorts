//! Hyper service implementation

use std::{fs::File, future::Future, io::Read, pin::Pin};

use http_body_util::Full;
use hyper::{
    body::{self, Bytes},
    service::Service,
    Method, Request, Response, StatusCode,
};
use hyper_tungstenite::is_upgrade_request;
use tokio::sync::mpsc::Sender;

use crate::{control::Controller, serve::registry::GAMES};

/// Service implementation responsible for handling routes and updating new controller connections
pub struct SpjortService {
    /// Controller send channel for connecting devices
    controller_sender: Sender<Controller>,
}

impl SpjortService {
    /// Creates a new spjortservice wrapping a controller sender
    pub fn new(controller_sender: Sender<Controller>) -> Self {
        Self { controller_sender }
    }
}

impl Service<Request<body::Incoming>> for SpjortService {
    type Response = Response<Full<Bytes>>;
    type Error = hyper::http::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<body::Incoming>) -> Self::Future {
        if is_upgrade_request(&req) {
            todo!()
        } else {
            let mut response = Response::builder();

            let res = match *req.method() {
                Method::GET => match req.uri().path() {
                    "/" => {
                        let mut buf = vec![];
                        let mut page =
                            File::open("frontend/index.html").expect("Failed to find file");
                        page.read_to_end(&mut buf)
                            .expect("Failed to read to buffer");
                        response
                            .status(StatusCode::OK)
                            .body(Full::new(Bytes::copy_from_slice(&buf)))
                    }
                    "/game" => {
                        let mut buf = vec![];
                        let mut page =
                            File::open("frontend/game.html").expect("Failed to find file");
                        page.read_to_end(&mut buf)
                            .expect("Failed to read to buffer");
                        response
                            .status(StatusCode::OK)
                            .body(Full::new(Bytes::copy_from_slice(&buf)))
                    }
                    "/games" => {
                        let games = GAMES
                            .iter()
                            .map(|game| game.render_html())
                            .collect::<Vec<_>>()
                            .join(" ");
                        response
                            .header("content-type", "application/json")
                            .status(StatusCode::OK)
                            .body(Full::new(Bytes::copy_from_slice(games.as_bytes())))
                    }
                    "/favicon.ico" => {
                        let mut buf = vec![];
                        let mut page =
                            File::open("frontend/favicon.ico").expect("Failed to find file");
                        page.read_to_end(&mut buf)
                            .expect("Failed to read to buffer");
                        response
                            .status(StatusCode::OK)
                            .body(Full::new(Bytes::copy_from_slice(&buf)))
                    }
                    fs if fs.starts_with("/frontend/") || fs.starts_with("/wasm") => {
                        let mut buf = vec![];
                        let mut page = File::open(&fs[1..]).expect("Failed to find file");
                        page.read_to_end(&mut buf)
                            .expect("Failed to read to buffer");
                        if fs.starts_with("/wasm") {
                            if fs.ends_with("js") {
                                response = response.header("content-type", "text/javascript");
                            } else if fs.ends_with("wasm") {
                                response = response.header("content-type", "application/wasm");
                            }
                        }

                        response
                            .status(StatusCode::OK)
                            .body(Full::new(Bytes::copy_from_slice(&buf)))
                    }
                    game if game.starts_with("/sports/") => {
                        let game = GAMES
                            .iter()
                            .find(|g| game.contains(g.name))
                            .expect("Valid game from query");
                        let game = game.render_game_scene();
                        response
                            .status(StatusCode::OK)
                            .body(Full::new(Bytes::copy_from_slice(game.as_bytes())))
                    }
                    _ => response
                        .status(StatusCode::NOT_FOUND)
                        .body(Full::new(Bytes::from_static(b"Not Found"))),
                },
                _ => response
                    .status(StatusCode::METHOD_NOT_ALLOWED)
                    .body(Full::new(Bytes::from_static(b"Method Not Allowed"))),
            };

            Box::pin(async { res })
        }
    }
}
