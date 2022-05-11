use std::time::Duration;
use pushover::API;
use pushover::requests::message::SendMessage;
use chrono::Local;
use rand::prelude::*;


fn send_message(msg: &str) {
    let api = API::new();

    let send_msg = SendMessage::new("a2y95cbxb45xv7vp1fdtoc54c9wqme", "u4jut4ezugoo1c6uubzkup4u5tzxaz", msg);
    let response = api.send(&send_msg);

    match response {
        Ok(response) => println!("{:?}", response),
        Err(err) => eprintln!("{:?}", err),
    }
}

#[tokio::main]
async fn main() {

    let mut i = tokio::time::interval(Duration::from_secs(5));

    let mut rng = thread_rng();

    loop {
        let minutes: u64 = rng.gen_range(5..40);

        tokio::time::sleep(Duration::from_secs(minutes * 60)).await;
        i.tick().await;
        let now = Local::now();
        let now = format!("{}", now.format("%H:%M"));
        send_message(&now);
    }

}
