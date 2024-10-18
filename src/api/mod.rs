use std::collections::HashMap;

const  API_URL: &str = "http://localhost:3000";

#[tokio::main]
pub async fn join_game(game_id: &String) -> Result<&str, Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{resp:#?}");
    Ok("hello")
}

#[tokio::main]
pub async fn score_point(game_id: &String, player_id: &String) -> Result<u16, Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{resp:#?}");
    Ok(8)
}