use yew::prelude::*;

use crate::app::components::html::HtmlRenderer;
use crate::app::components::page::Page;
use crate::data::CONTACT;

#[function_component]
pub fn ContactView() -> Html {
    let html = CONTACT.html.to_string();
    html! {
        <Page>
            <div class="content-container">
                <HtmlRenderer html={html}/>
            </div>
        </Page>
    }
}
