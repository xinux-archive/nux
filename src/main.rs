mod package;
use serde::{Serialize, Deserialize};
use reqwest::{Client};
use tokio::main;
use package::request::Type;
use crate::package::request::Request;
use crate::package::response::Response;

#[tokio::main]
async fn main() {
    let new_type = Request::new("asc");

    let new_type: Response = Client::new()
        .post("https://search.nixos.org/packages?channel=23.05&size=50&sort=alpha_asc&type=options&query=Pack")
        .json(&new_type)
        .send()
        .await.expect("Failed to send")
        .json()
        .await.expect("Failed to parse");

    println!("{:#?}", new_type);
}