// let url = "http://localhost:5000/test";

#![allow(dead_code)]
use reqwest::blocking::get;
use reqwest::header::{CONTENT_LENGTH, COOKIE};
// #[tokio::main]
pub async fn get_request() -> Result<(), Box<dyn std::error::Error>> {
    tokio::task::spawn_blocking(move || blocking_get().unwrap());
    //basic().await?;
    //json().await?;
    //post().await?;
    //status().await?;
    //request().await?;
    //cookies().await?;
    //proxy().await?;

    Ok(())
}

fn blocking_get() -> Result<(), Box<dyn std::error::Error>> {
    let res = get("127.0.0.1:5000")?;

    let body = res.text()?;
    println!("body = {:?}", body);

    Ok(())
}
