extern crate core;
extern crate hyper;
extern crate serde;

use serde::{Deserialize, Serialize};

use core::task::Poll;

use rustc_serialize::json;
use rustc_serialize::{Encodable, Encoder};
use std::io::Read;
use std::str::FromStr;

use hyper::client::{Client, HttpConnector};
use hyper::{Body, Request, Response, StatusCode, Uri};

pub const SERVER_ADDR: &'static str = "127.0.0.1:1980";
pub const BOT_ADDR: &'static str = "127.0.0.1:1981";
pub const HTML_ADDR: &'static str = "http://127.0.0.1:1980";

pub const HTML_DATA: &'static str = "data/index.html";
pub const HTML_HEADER: &'static str = "html/header.html";
pub const HTML_FOOTER: &'static str = "html/footer.html";

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub user: String,
    pub text: String,
}

impl Message {
    pub fn new(user: String, text: String) -> Message {
        Message {
            text: text,
            user: user,
        }
    }
}

pub struct UserClient {
    username: String,
    server_addr: String,
    client: Client<HttpConnector, Body>,
}

impl UserClient {
    pub fn new(username: String, server_addr: String) -> UserClient {
        UserClient {
            username: username,
            server_addr: server_addr,
            client: Client::new(),
        }
    }

    // TODO: Implement send_msg
    pub async fn send_msg(&self, msg: String) -> Result<(), hyper::Error> {
        let uri = Uri::from_str(&self.server_addr.as_str()).unwrap();
        let msg = Message::new(String::from(self.username.as_str()), msg);
        let mb = serde_json::to_string(&msg).unwrap();
        let req = Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::from(mb))
            .expect("request builder for message");

        let res = self.client.request(req).await?;

        println!("{}", res.status());
        Ok(())
    }

    pub async fn get_content(&self) -> Result<Response<Body>, hyper::Error> {
        let uri = Uri::from_str(&self.server_addr.as_str()).unwrap();
        let res = self.client.get(uri).await?;
        println!("{}", res.status());
        Ok(res)
    }
}
