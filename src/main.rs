#![allow(non_snake_case)]
#[macro_use]
extern crate dotenv_codegen;
extern crate num_cpus;

mod discord;
mod download;

use std::{io};
use dotenv_codegen::dotenv;
use std::env;
use std::str::FromStr;

#[tokio::main]
async fn main() {

    //let oki = discord::getImages("557733205428666378".to_string(), 1000).await;

    let mut token = String::from(dotenv!("token"));
    let mut channelID = String::new();
    let mut amount = String::new();
    println!("{}",token);
    if token.is_empty() {

        println!("Input user token: ");
        io::stdin()
            .read_line(&mut token)
            .expect("Do better");
    }
    println!("Input channel ID: ");
    io::stdin()
        .read_line(&mut channelID)
        .expect("Do better");
    println!("Input message amount: ");
    io::stdin()
        .read_line(&mut amount)
        .expect("Do better");
    let new_amount = amount.drain(..amount.len()-1);
    let new_cid = channelID.drain(..channelID.len()-1);
    let oki = discord::getImages(new_cid.as_str().to_string(), i32::from_str( new_amount.as_str()).expect("GIVE NUMBER BAAKAAAAAAAA"), &token).await;

    println!("{:?}", oki.is_ok());
}