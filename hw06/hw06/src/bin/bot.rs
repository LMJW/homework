extern crate bbs;
extern crate hyper;
extern crate rand;
extern crate rustc_serialize;
extern crate tokio;

use futures::executor::block_on;
use std::io::{self, Read};
use std::net::TcpListener;

// use hyper::header::UserAgent;
use hyper::Client;

use bbs::UserClient;
use bbs::{BOT_ADDR, HTML_ADDR};

use rand::prelude::*;

#[tokio::main]
async fn main() -> io::Result<()> {
    // Create a bot user.
    let bot = UserClient::new("bot".to_string(), HTML_ADDR.to_string());

    // Start TcpListener.
    let lis = TcpListener::bind(BOT_ADDR)?;

    // Listen for incoming TCP connections.
    // For each connection, accept it and read all data from the stream.
    // If it's a 'choose' message, post to the BBS with a response (via the above bot user).
    for stream in lis.incoming() {
        let mut buf = String::new();
        match stream {
            Ok(mut s) => {
                s.read_to_string(&mut buf);
                println!("tcp: {}", buf);
                println!("read successful");
                if rand::random::<bool>() {
                    let m = async {
                        println!("will reply:");
                        let res = bot.send_msg("received!".to_string()).await.unwrap();
                        println!("message sent");
                    };
                    block_on(m);
                };
            }
            Err(e) => println!("error : {:#?}", e),
        }
    }
    Ok(())
}
