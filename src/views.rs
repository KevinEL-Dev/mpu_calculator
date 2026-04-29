use maud::{html, Markup, DOCTYPE,PreEscaped};

pub async fn hello_world() -> Markup {
    html! {
        (DOCTYPE)
        script src="/static/htmx.min.js" {}
        h1 {"PPMC"}

        input class="form-control" type="search" name="pattern" placeholder="Begin typing to search for meals..." hx-get="/search_meals" hx-params="*" hx-trigger="input changed delay:500ms, keup[key=='Enter'], load" hx-target="#meal_result" {}
        div id="meal_result" {}
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
            label for="measurement_unit_name" {"Measurement unit"}
            br;
            input class="form-control" type="text" list="measurements" name="measurement_unit_name" hx-get="/search_measurement_units" hx-params="*" hx-trigger="input changed delay:500ms, keyup[key=='Enter'], load" hx-target="#measurements" hx-swap="innerHTML"{}
            datalist id="measurements" {}
            br;
            input type="submit" value="Submit";
        }
        div id="result" {}
        br;
        form hx-post="/create_measurement_unit" hx-target="#measurement_unit_result"{
            label for="name" {"Measurement Unit Name"}
            br;
            input type="text" id="name" name="name";
            br;
            input type="submit" value="Submit";
        }
        div id="measurement_unit_result" {}
        br;
        form hx-post="/create_meal" hx-target="#meal_result"{
            label for="name" {"Meal Name"}
            br;
            input type="text" id="name" name="name";
            br;
            input type="submit" value="Submit";
        }
        div id="meal_result" {}
    }
}
pub async fn get_register() -> Markup {
    html! {
        (DOCTYPE)
        h1 {"register"}
        script src="/static/htmx.min.js" {}
        form hx-post="/register" hx-target="#result" hx-swap="innerHTML"{
            label for="username" {"Username"}
            br;
            input type="text" id="username" name="username";
            br;
            label for="password" {"Password"}
            br;
            input type="password" id="password" name="password";
            br;
            input type="submit" value="Submit";
        }
        div id="result" {}
    }
}
pub fn render_measurement_unit_search(output: &str) -> Markup {
    html!{
        (PreEscaped(output))
    }
}
pub fn render_meal_search(output: &str) -> Markup {
    html!{
        (PreEscaped(output))
    }
}
pub fn render_source_search(output: &str) -> Markup {
    html!{
        (PreEscaped(output))
    }
}
