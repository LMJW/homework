extern crate core;
extern crate hyper;
extern crate rustc_serialize;

use core::task::Poll;

use rustc_serialize::json;
use rustc_serialize::{Encodable, Encoder};
use std::io::Read;
use std::str::FromStr;

use hyper::client::{Client, HttpConnector};
use hyper::{Body, StatusCode, Uri};

pub const SERVER_ADDR: &'static str = "127.0.0.1:1980";
pub const BOT_ADDR: &'static str = "127.0.0.1:1981";
pub const HTML_ADDR: &'static str = "http://127.0.0.1:1980";

pub const HTML_DATA: &'static str = "data/index.html";
pub const HTML_HEADER: &'static str = "html/header.html";
pub const HTML_FOOTER: &'static str = "html/footer.html";

#[derive(Debug, RustcDecodable, RustcEncodable)]
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

    pub async fn get_content(&self) -> Result<(), hyper::Error> {
        let uri = Uri::from_str(&self.server_addr.as_str()).unwrap();
        let res = self.client.get(uri).await?;
        println!("{}", res.status());
        Ok(())
    }
}
