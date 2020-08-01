use k8s_openapi::api::core::v1::Event;
use reqwest::Client;
use serde::{Deserialize, Serialize};

fn format_message(event: Event) -> TeamsMessage {
    TeamsMessage {
        message_type: "MessageCard",
        context: "http://schema.org/extensions",
        theme_color: "0076D7",
        summary: event,
    }
}

pub fn send_message(url: &str, event: Event) {
    let msg = format_message(event);
    let resp = Client::new().json(&msg).send().await()?.json().await()?;
    todo!()
}

#[derive(Serialize, Deserialize)]
struct TeamsMessage {
    #[serde(rename = "@type")]
    message_type: String,
    #[serde(rename = "@context")]
    context: String,
    #[serde(rename = "themeColor")]
    theme_color: String,
    summary: String,
    sections: Vec<TeamsMessageSection>,
}

struct TeamsMessageSection {
    #[serde(rename = "activityTitle")]
    activity_title: String,
    #[serde(rename = "activitySubtitle")]
    activity_subtitle: String,
    #[serde(rename = "activityImage")]
    activity_image: String,
    facts: Vec<TeamsMessageSectionFact>,
    markdown: bool,
}

struct TeamsMessageSectionFact {
    name: String,
    value: String,
}
