use yew::prelude::*;

use super::utils::set_meta;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    use_effect(|| {
        set_meta("404 | My Profile", "Page not found.");
        || ()
    });

    html! {
        <div class="main-container">
            <h1 class="text-primary fw-bold">{ "404 - Not Found" }</h1>
        </div>
    }
}
