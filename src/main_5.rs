// use std::{thread::sleep, time::Duration};

#[tokio::main]
async fn main() {
    let r = fetch_url().await.unwrap();
    println!("{}", r);
}

// async fn say_hello() -> u32 {
//     sleep(Duration::from_secs(3));
//     5
// }

// async fn say_good_afternoon() -> u32 {
//     sleep(Duration::from_secs(4));
//     6
// }

async fn fetch_url() -> Result<String, reqwest::Error> {
    let r = reqwest::get("https://www.undercoverdevs.com")
        .await?
        .text()
        .await?;
    Ok(r)
}
