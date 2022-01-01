#![allow(non_snake_case)]

use std::path::Path;
use json::JsonValue;
use std::sync::mpsc;
use std::thread;
use std::mem;

pub async fn multithread_queue(queue: Vec<JsonValue>) {
    let (tx, rx) = mpsc::channel();
    for i in 0..queue.len() {
        let tx = tx.clone();
        let data = queue[i].clone();
        thread::spawn(move || { // Find a way to use/move the 2 functions in a thread to be used
            //parse_json(&data).await;
            
            tx.send(i).unwrap(); // Add some type of error check and send to main thread
        });
    }

    mem::drop(tx);

    for r in rx {
        println!("{} finished task.", r);
    } // In case of error from thread, kill it/join it, then make new thread for that task (might be lazy and just kill all threads then try agian)
}

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
