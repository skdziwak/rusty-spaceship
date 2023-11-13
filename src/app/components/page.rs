use yew::prelude::*;

use crate::app::components::nav_bar::NavBar;

#[derive(Properties, PartialEq)]
pub struct PageProperties {
    pub children: Children,
}

#[function_component]
pub fn Page(props: &PageProperties) -> Html {
    html! {
        <>
            <div class="full-screen-background">
                <NavBar/>
                <div class="content">
                    { props.children.clone() }
                </div>
            </div>
        </>
    }
}
