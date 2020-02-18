extern crate bbs;
extern crate hyper;
extern crate rustc_serialize;
extern crate tokio;

use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::prelude::*;

use bbs::Message;
use bbs::{BOT_ADDR, HTML_DATA, HTML_FOOTER, HTML_HEADER, SERVER_ADDR};
use hyper::server::Server;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, StatusCode};
use rustc_serialize::json;

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

async fn req_handler(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match *req.method() {
        Method::GET => {
            // Read the files [HTML_HEADER, HTML_DATA, HTML_FOOTER] into buf.
            // If HTML_DATA doesn't exist, it should be skipped without failure.
            // Use `try_or_server_err!(expression, res)` instead of `try!(expression)` in
            // order to return an internal server error.
            let mut buf = String::new();
            // TODO

            // And return buf as the response.
            let ret = Response::new(Body::from(buf));
            Ok(ret)
        }
        Method::POST => {
            // Read the message out of the `req` into a buffer, handle it, and respond with Ok.
            // TODO
            unimplemented!()
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
