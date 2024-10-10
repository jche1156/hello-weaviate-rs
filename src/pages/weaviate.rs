use anyhow::Result;
use maud::{html, Markup};
use std::env;
use std::error::Error;
use weaviate_community::collections::query::GetQuery;
use weaviate_community::WeaviateClient;
async fn querying(client: WeaviateClient) -> Result<(), Box<dyn Error>> {
    // Get
    let query = GetQuery::builder("Question", vec!["category", "question", "answer"])
        .with_near_text("{concepts: \"genus\"}")
        .with_limit(1)
        .build();
    let res = client.query.get(query).await?;
    println!("{:#?}", res);

    Ok(())
}

async fn weave() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;
    let weaviate_endpoint = env::var("WEAVIATE_URL")?;
    let weaviate_key = env::var("WEAVIATE_API_KEY")?;
    let oai_key = env::var("OPENAI_API_KEY")?;

    let client = WeaviateClient::builder(weaviate_endpoint.as_str())
        .with_auth_secret(weaviate_key.as_str())
        .with_api_key("X-OpenAI-Api-Key", oai_key.as_str())
        .build()?;

    // Check database is live
    let res = client.is_live().await?;
    println!("Database Live? :{}", res);
    // Check database is ready
    let res = client.is_ready().await?;
    println!("Database Ready? :{}", res);

    if let Err(e) = querying(client).await {
        eprintln!("Error: {}", e);
    }

    Ok(())
}

pub async fn render() -> Markup {
    if let Err(e) = weave().await {
        eprintln!("Error: {}", e);
    }
    html! {
        h1 { "Weaviate" }
    }
}
