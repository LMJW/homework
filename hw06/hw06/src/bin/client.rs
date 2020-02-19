extern crate bbs;
#[macro_use]
extern crate clap;

extern crate hyper;
extern crate serde;
extern crate tokio;

use bbs::{UserClient, HTML_ADDR};
use clap::{App, Arg, SubCommand};
use futures::executor::block_on;
use serde::{Deserialize, Serialize};
use std::env::args;

#[tokio::main]
async fn main() {
    let m = App::new("client messaging app")
        .author("LMJW")
        .version("1.0.0")
        .about("A messaging app that can send message to server")
        .arg(
            Arg::with_name("address")
                .short("a")
                .help(format!("Default address: {}", HTML_ADDR).as_str()),
        )
        .subcommands(vec![
            SubCommand::with_name("post")
                .about("post a message to server")
                .arg(
                    Arg::with_name("username")
                        .short("u")
                        .long("username")
                        .takes_value(true)
                        .required(true)
                        .help("specify the username"),
                )
                .arg(
                    Arg::with_name("message")
                        .short("m")
                        .long("message")
                        .takes_value(true)
                        .required(true)
                        .help("the message content"),
                ),
            SubCommand::with_name("get").about("get message from server"),
        ])
        .get_matches();

    let addr = m.value_of("address").unwrap_or(HTML_ADDR);

    if let Some(cmd) = m.subcommand_matches("post") {
        let user = cmd.value_of("username").unwrap();
        let msg = cmd.value_of("message").unwrap();
        println!("user: {}; message: {};", &user, &msg);
        let m = async {
            println!("start");
            let c = UserClient::new(user.to_string(), addr.to_string());
            if let Ok(res) = c.send_msg(msg.to_string()).await {
                println!("successfully sent the message")
            }
        };
        block_on(m);
        println!("Done");
    } else if let Some(cmd) = m.subcommand_matches("get") {
        let m = async {
            let c = UserClient::new("".to_string(), addr.to_string());
            if let Ok(res) = c.get_content().await {
                let body = hyper::body::to_bytes(res.into_body()).await.unwrap();
                let buf: Vec<u8> = body.to_vec();
                let buf = String::from_utf8(buf).unwrap();
                println!("{:#?}", buf.as_str());
            }
        };
        block_on(m);
        println!("Done");
    }
}
