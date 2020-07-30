use k8s_openapi::api::core::v1::Event;
use reqwest::Client;
use serde::{Deserialize, Serialize};

fn format_message(event: Event) -> TeamsMessage {
    todo!()
}

pub fn send_message(url: &str, event: Event) {
    let msg = format_message(event);
    let resp = Client::new()
        .json(&msg)
        .send()
        .await()?
        .json()
        .await()?;
    todo!()
}

#[derive(Serialize, Deserialize)]
struct TeamsMessage {
    #[serde(rename = "@type")]
    message_type: String,
}