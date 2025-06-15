use yew::prelude::*;

use super::utils::set_meta;

#[function_component(Home)]
pub fn home() -> Html {
    use_effect(|| {
        set_meta("Home | My Profile", "Welcome to my profile page.");
        || ()
    });

    html! {
        <div class="main-container">
            <h1 class="text-primary fw-bold">{ "Welcome👋" }</h1>
        </div>
    }
}
