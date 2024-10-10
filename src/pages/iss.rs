use anyhow::Result;
use maud::{html, Markup};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
struct Report {
    title: String,
    summary: String,
    news_site: String,
    url: String,
}

#[derive(Deserialize)]
struct Reports {
    results: Vec<Report>,
}

async fn iss() -> Result<Reports> {
    let body = reqwest::get("https://api.spaceflightnewsapi.net/v4/reports/?format=json&limit=1")
        .await?
        .text()
        .await?;
    let r: Reports = serde_json::from_str(&body)?;
    Ok(r)
}

pub async fn render() -> Markup {
    let report = iss().await.unwrap().results[0].clone();
    html! {
        h3 { (report.title) }
        subtitle { (report.news_site) }
        p {
            span { (report.summary) }
            a href=(report.url) { "Read More" }
        }
    }
}
