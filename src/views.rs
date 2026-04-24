use maud::{html, Markup, DOCTYPE};

pub async fn hello_world() -> Markup {
    html! {
        (DOCTYPE)
        script src="/static/htmx.min.js" {}
        h1 {"PPMC"}

/*        form hx-get="/search_sources" hx-target="#result"{
            label for="pattern" {"Search Source "}
            input id="pattern" name="pattern" type="text" {}
            input type="submit" value="Submit" {}
        }
        input class="form-control" type="search" name="pattern" placeholder="Begin typing to search for sources..." hx-get="/search_sources" hx-params="*" hx-trigger="input changed delay:500ms, keup[key=='Enter'], load" hx-target="#result" hx-indicator=".htmx-indicator" {}
        div id="result";*/

        form hx-post="/create_source" hx-target="#result"{
            label for="source_name" {"Source name"}
            br;
            input type="text" id="name" name="name";
            br;
            label for="brand" {"Brand name"}
            br;
            input type="text" id="brand" name="brand";
            br;
            label for="source_price" {"Price"}
            br;
            input type="number" step="0.01" min="0" id="price" name="price";
            br;
            label for="servings_per_container" {"Servings per container"}
            br;
            input type="number" step="0.01" min="0" id="servings_per_container" name="servings_per_container";
            br;
            label for="serving_size" {"Serving Size"}
            br;
            input type="number"  step="0.01" min="0" id="serving_size" name="serving_size";
            br;
            label for="measurement_unit" {"Measurement unit"}
            br;
            input type="number" step="1" id="measurement_unit_id" name="measurement_unit_id";
            br;
            input type="submit" value="Submit";
        }
        div id="result";
    }
}
