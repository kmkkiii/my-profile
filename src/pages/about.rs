use yew::prelude::*;

use crate::components::profile::Profile;

use super::utils::set_meta;

#[function_component(About)]
pub fn about() -> Html {
    use_effect(|| {
        set_meta("About | My Profile", "Learn more about me.");
        || ()
    });

    html! {
        <div class="main-container">
            <h1 class="text-primary fw-bold">{ "About Me" }</h1>
            <div class="mb-4">
                <Profile name="Ryosuke Komaki" bio="Web Application Developer" />
            </div>
        </div>
    }
}
