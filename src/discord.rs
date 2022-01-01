#![allow(non_snake_case)]

use json::JsonValue;

use crate::download;

pub async fn getImages(chanID: String, amount: i32, token: &str) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let mut queue: Vec<JsonValue> = Vec::new();

    if amount <= 100 {
        let url = format!("https://canary.discord.com/api/v9/channels/{}/messages?limit=100", chanID);
        let res = client
            .get(url)
            .header("authorization", token)
            .send()
            .await.unwrap();

        let data = json::parse(&res.text().await.unwrap()).unwrap();
        queue.push(data.clone());
        //download::parse_json(&data).await;
    } else {
        let url = format!("https://canary.discord.com/api/v9/channels/{}/messages?limit=100", chanID);
        let res = client
            .get(url)
            .header("authorization", token)
            .send()
            .await.unwrap();

        let data = json::parse(&res.text().await.unwrap()).unwrap();
        //download::parse_json(&data).await;
        queue.push(data.clone());

        let mut lastMsg = data.members().last().unwrap()["id"].clone();
        let amountHund = f64::from(amount / 100).floor() - 1_f64;
        let amountTens = amount % 100;

        for _i in 0..amountHund as i32 {
            let url = format!("https://canary.discord.com/api/v9/channels/{}/messages?limit=100&before={}", chanID, lastMsg);
            let res = client
                .get(url)
                .header("authorization", token)
                .send()
                .await.unwrap();

            let dataHund = json::parse(&res.text().await.unwrap()).unwrap();
            queue.push(dataHund.clone());
            //download::parse_json(&dataHund).await;

            lastMsg = data.members().last().unwrap()["id"].clone();
        }

        let url = format!("https://canary.discord.com/api/v9/channels/{}/messages?limit={}&before={}", chanID, amountTens, lastMsg);
        let res = client
            .get(url)
            .header("authorization", token)
            .send()
            .await.unwrap();

        let dataTens = json::parse(&res.text().await.unwrap()).unwrap();
        queue.push(dataTens.clone());
        //download::parse_json(&dataTens).await;
    }

    download::multithread_queue(queue).await;

    Ok(())
}

