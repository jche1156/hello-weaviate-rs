use maud::{html, Markup};

pub async fn render() -> Markup {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    html! {
        h1 { ("Version") }
        p {
            { (VERSION) }
        }
    }
}
