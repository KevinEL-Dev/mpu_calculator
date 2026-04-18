use maud::{html, Markup};

pub async fn hello_world() -> Markup {
    html! {
        h1 {"Hello its me world."}
    }
}
