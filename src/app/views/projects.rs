use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::app::{components::{page::Page, grid_tile::{GridTile, GridTileProps}}, views::Route};
use crate::data::PROJECTS;

#[function_component]
pub fn ProjectsView() -> Html {
    let navigator = use_navigator().unwrap();
    let tiles: Vec<Html> = PROJECTS.iter().map(|project| {
        let navigator = navigator.clone();
        let go_readme = Callback::from(move |_| navigator.push(&Route::Readme { path: project.path.to_string() }));
        let props = GridTileProps {
            title: project.name.to_string(),
            technologies: project.technologies.iter().map(|tech| tech.to_string()).collect(),
            image: project.thumbnail.to_string(),
            on_click: go_readme,
        };
        html! {
            <GridTile ..props/>
        }
    }).collect();
    html! {
        <Page>
            <div class="grid-container">
                <h1 id="grid-tiles-title">{"Projects"}</h1>
                <div class="grid-tiles">
                    {tiles}
                </div>
            </div>
        </Page>
    }
}
