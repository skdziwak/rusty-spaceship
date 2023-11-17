use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::app::views::Route;
use crate::app::components::page::Page;

#[function_component]
pub fn MainView() -> Html {
    let navigator = use_navigator().unwrap();
    let go_contact = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&Route::Contact))
    };
    let go_projects = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&Route::Projects))
    };
    html! {
        <Page>
            <div class="title-container">
                <h1>{"rusty spaceship"}</h1>
                <h2>{"Rust, Go and C++ portfolio of Szymon Dziwak"}</h2>
                <div class="button-container">
                    <button class="cta-button" id="see-work" onclick={go_projects}>{"See My Work"}</button>
                    <button class="cta-button" id="hire-me" onclick={go_contact}>{"Hire Me"}</button>
                </div>
            </div>
        </Page>
    }
}
