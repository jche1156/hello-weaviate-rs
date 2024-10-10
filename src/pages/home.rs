use maud::{html, Markup};

pub async fn render() -> Markup {
    html! {
        h1 { "Hello, World!" }
    }
}
