#![allow(non_snake_case)]

use std::path::Path;
use json::JsonValue;

pub async fn parse_json(json: &JsonValue) {
    for message in json.members() {
        for attachment in message["attachments"].members() {
            println!("Attachment: {}", attachment);
            download_url(attachment["url"].as_str().unwrap()).await;
        }
    }
}

pub async fn download_url(url:&str) {
    // let mut byt = reqwest::get(url).await.expect("unable to download img").bytes();
    // let mut content = Cursor::new(byt.await);
    let pa = Path::new("../../Imgs");
    // let mut file = std::fs::File::create(pa.join(format!("./{}", self.downloaded))).expect("unable to make file");
    let mut dest = std::fs::File::create(pa.join(format!("./{}.{}", uuid::Uuid::new_v4(), url[url.rfind(".").unwrap_or(0)..].to_string()))).expect("unable to make file");
    println!("{:#?}",dest);
    let src = reqwest::get(url).await.expect("unable to make request").bytes().await.expect("unable to get bytes");
    std::io::copy(&mut src.as_ref(), &mut dest).expect(&*format!("unable to write to file for {}", url));
}
