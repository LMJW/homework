extern crate bbs;
extern crate hyper;
extern crate rustc_serialize;

use std::io::Read;
use std::net::TcpListener;

// use hyper::header::UserAgent;
use hyper::Client;

use bbs::UserClient;
use bbs::{BOT_ADDR, HTML_ADDR};

fn main() {
    // Create a bot user.
    // TODO

    // Start TcpListener.
    // TODO

    // Listen for incoming TCP connections.
    // For each connection, accept it and read all data from the stream.
    // If it's a 'choose' message, post to the BBS with a response (via the above bot user).
    // TODO
}
