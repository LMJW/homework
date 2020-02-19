extern crate bbs;
extern crate hyper;
extern crate rustc_serialize;
extern crate serde;
extern crate tokio;

use std::error::Error;
use std::fmt;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::net::TcpStream;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::prelude::*;

use bbs::Message;
use bbs::{BOT_ADDR, HTML_DATA, HTML_FOOTER, HTML_HEADER, SERVER_ADDR};
use hyper::server::Server;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, StatusCode};

// deperated crate, change to serde
// use rustc_serialize::json;

// Returns val from Ok(val) or sets the response to return an InternalServerError.
macro_rules! try_or_server_err {
    ($expr:expr, $res:expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => {
                println!("{:?}", err);
                *($res).status_mut() = StatusCode::InternalServerError;
                return;
            }
        }
    };
}

#[derive(Debug)]
pub enum HandlerError {
    IoError(io::Error),
    HyperError(hyper::Error),
    JsonError(serde_json::error::Error),
}

impl From<io::Error> for HandlerError {
    fn from(e: io::Error) -> Self {
        HandlerError::IoError(e)
    }
}

impl From<hyper::Error> for HandlerError {
    fn from(e: hyper::Error) -> Self {
        HandlerError::HyperError(e)
    }
}

impl From<serde_json::error::Error> for HandlerError {
    fn from(e: serde_json::error::Error) -> Self {
        HandlerError::JsonError(e)
    }
}

impl Error for HandlerError {
    fn description(&self) -> &str {
        match &self {
            HandlerError::IoError(_) => "io error",
            HandlerError::HyperError(_) => "hyper error",
            HandlerError::JsonError(_) => "json error",
        }
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self {
            HandlerError::IoError(e) => Some(e),
            HandlerError::HyperError(e) => Some(e),
            HandlerError::JsonError(e) => Some(e),
        }
    }
}
impl fmt::Display for HandlerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HandlerError is here!")
    }
}

async fn req_handler(req: Request<Body>) -> Result<Response<Body>, HandlerError> {
    match *req.method() {
        Method::GET => {
            // Read the files [HTML_HEADER, HTML_DATA, HTML_FOOTER] into buf.
            // If HTML_DATA doesn't exist, it should be skipped without failure.
            // Use `try_or_server_err!(expression, res)` instead of `try!(expression)` in
            // order to return an internal server error.
            let mut buf = String::new();
            let header = fs::read_to_string(HTML_HEADER)?;
            let footer = fs::read_to_string(HTML_FOOTER)?;

            let data = fs::read_to_string(HTML_DATA);

            buf.push_str(header.as_str());
            match data {
                Ok(d) => {
                    buf.push_str(d.as_str());
                }
                Err(e) => {
                    println!("HTML_DATA does not exist: {}", e);
                }
            }

            buf.push_str(footer.as_str());

            // And return buf as the response.
            let ret = Response::new(Body::from(buf));
            Ok(ret)
        }
        Method::POST => {
            // Read the message out of the `req` into a buffer, handle it, and respond with Ok.

            let body = hyper::body::to_bytes(req.into_body()).await?;
            let buf: Vec<u8> = body.to_vec();

            let msg: Message = serde_json::from_slice(&buf)?;
            let mut html = String::new();
            html.push_str(format!("<p>{}: {}</p>\n", msg.user, msg.text).as_str());
            // fs::write(HTML_DATA, &buf)?;
            let mut f = match fs::OpenOptions::new().append(true).open(HTML_DATA) {
                Ok(f) => f,
                Err(e) => match e.kind() {
                    std::io::ErrorKind::NotFound => fs::OpenOptions::new()
                        .write(true)
                        .create(true)
                        .open(HTML_DATA)
                        .unwrap(),
                    _ => panic!(e),
                },
            };
            f.write(&html.as_bytes());

            // relay message to bot, to see if bot want to reply
            // create tcp client and send message to bot port
            let mut stream = TcpStream::connect(BOT_ADDR)?;
            stream.write(&buf)?;

            let mut res = Response::new(Body::empty());
            *res.status_mut() = StatusCode::OK;
            Ok(res)
        }
        _ => {
            let mut res = Response::new(Body::empty());
            *res.status_mut() = StatusCode::IM_A_TEAPOT;
            Ok(res)
        }
    }
}

#[tokio::main]
async fn main() {
    println!("Listening on {}.", SERVER_ADDR);

    let addr: SocketAddr = (*SERVER_ADDR).parse::<SocketAddr>().unwrap();

    let service = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(req_handler)) });

    let server = Server::bind(&addr).serve(service);

    if let Err(e) = server.await {
        eprintln!("server error :{}", e);
    }
}
