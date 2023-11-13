#![allow(non_camel_case_types)]
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct GridTileProps {
    pub image: String,
    pub title: String,
    pub technologies: Vec<String>,
    pub on_click: Callback<()>,
}

#[function_component(GridTile)]
pub fn grid_tile(props: &GridTileProps) -> Html {
    let on_click = {
        let signal = props.on_click.clone();
        move |_| signal.emit(())
    };
    html! {
        <div class="grid-tile" onclick={on_click}>
            <div class="thumbnail">
                <img src={props.image.clone()} />
            </div>
            <div class="title">{props.title.clone()}</div>
            <div class="technologies">
                { for props.technologies.iter().map(|tech| html!{ <span>{ tech }</span> }) }
            </div>
        </div>
    }
}
