use std::collections::VecDeque;
use std::{env, process};

use su::client::run_client;
use su::server::run_server;

#[tokio::main]
async fn main() {
    let args = env::args();
    let mut args_deque: VecDeque<String> = args.collect();
    //pop first
    args_deque.pop_front().unwrap();

    let second = args_deque.pop_front();
    if let Some(second) = second {
        if second == "--daemon" {
            run_server().await;
            process::exit(0);
        }
    }
    run_client().await;
}
