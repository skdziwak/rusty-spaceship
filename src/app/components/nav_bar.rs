use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::app::views::Route;

#[function_component]
pub fn NavBar() -> Html {
    let navigator = use_navigator().unwrap();
    let go_home = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&Route::Main))
    };
    let go_about = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&Route::About))
    };
    let go_projects = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&Route::Projects))
    };
    let go_contact = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&Route::Contact))
    };
    html! {
        <nav class="nav-bar">
            <div class="nav-container">
                <a class="nav-link" onclick={go_home}>{"Home"}</a>
                <a class="nav-link" onclick={go_about}>{"About"}</a>
                <a class="nav-link" onclick={go_projects}>{"Projects"}</a>
                <a class="nav-link" href="https://github.com/skdziwak" target="_blank">{"GitHub"}</a>
                <a class="nav-link" href="https://www.linkedin.com/in/szymon-dziwak/" target="_blank">{"LinkedIn"}</a>
                <a class="nav-link" onclick={go_contact}>{"Contact"}</a>
            </div>
        </nav>
    }
}
