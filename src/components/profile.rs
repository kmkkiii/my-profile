use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ProfileProps {
    pub name: String,
    pub bio: String,
}

#[function_component(Profile)]
pub fn profile(props: &ProfileProps) -> Html {
    let show_info = use_state(|| false);

    let toggle_info = {
        let show_info = show_info.clone();
        Callback::from(move |_| show_info.set(!*show_info))
    };

    let button_message = if *show_info {
        "Hide Profile"
    } else {
        "Show Profile"
    };

    html! {
        <div class="custom-card">
            <h2 class="fw-bold">{ "Profile" }</h2>
            <button class="btn btn-custom mt-2" onclick={toggle_info.clone()}>
                { button_message }
            </button>
            if *show_info {
                <div class="mt-2">
                    <h3>{ &props.name }</h3>
                    <p class="text-muted">{ &props.bio }</p>
                </div>
            }
        </div>
    }
}
