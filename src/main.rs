use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod pages;

use pages::home::Home;
use pages::about::About;
use pages::not_found::NotFound;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(NavBar)]
fn navbar() -> Html {
    html! {
        <nav class="navbar navbar-expand-lg navbar-dark bg-primary">
            <div class="container">
                <Link<Route> classes="navbar-brand fw-bold" to={Route::Home}>{"My Profile"}</Link<Route>>
                <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarNav">
                    <span class="navbar-toggler-icon"></span>
                </button>
                <div class="collapse navbar-collapse" id="navbarNav">
                    <ul class="navbar-nav ms-auto">
                        <li class="nav-item"><Link<Route> classes="nav-link" to={Route::Home}>{"Home"}</Link<Route>></li>
                        <li class="nav-item"><Link<Route> classes="nav-link" to={Route::About}>{"About"}</Link<Route>></li>
                    </ul>
                </div>
            </div>
        </nav>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::About => html! { <About /> },
        Route::NotFound => html! { <NotFound /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <NavBar />
            <div class="container">
                <Switch<Route> render={switch} />
            </div>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
