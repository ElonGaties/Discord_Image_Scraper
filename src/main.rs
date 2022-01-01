#![allow(non_snake_case)]

mod discord;
mod download;

use std::io;

#[tokio::main]
async fn main() {
    //let oki = discord::getImages("557733205428666378".to_string(), 1000).await;

    let mut token = String::new();
    let mut channelID = String::new();
    let mut amount = String::new();

    println!("Input user token: ");
    io::stdin()
        .read_line(&mut token)
        .expect("Do better");
    println!("Input channel ID: ");
    io::stdin()
        .read_line(&mut channelID)
        .expect("Do better");
    println!("Input message amount: ");
    io::stdin()
        .read_line(&mut amount)
        .expect("Do better");
    
    let oki = discord::getImages(channelID, amount.parse::<i32>().unwrap(), token.as_str()).await;

    println!("{:?}", oki.is_ok());
}