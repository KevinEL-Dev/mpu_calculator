use maud::{html, Markup, DOCTYPE};

pub async fn hello_world() -> Markup {
    html! {
        (DOCTYPE)
        script src="/static/htmx.min.js" {}
        h1 {"PPMC"}

        form hx-get="/search_sources" hx-target="#result"{
            label for="pattern" {"Search Source "}
            input id="pattern" name="pattern" type="text" {}
            input type="submit" value="Submit" {}
        }
        input class="form-control" type="search" name="pattern" placeholder="Begin typing to search for sources..." hx-get="/search_sources" hx-params="*" hx-trigger="input changed delay:500ms, keup[key=='Enter'], load" hx-target="#result" hx-indicator=".htmx-indicator" {}
        div id="result";
    }
}
