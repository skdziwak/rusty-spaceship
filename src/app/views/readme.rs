#![allow(non_camel_case_types)]
use yew::prelude::*;

use super::ProjectId;
use crate::app::components::markdown::MarkdownRenderer;
use crate::app::components::page::Page;
use crate::data::PROJECTS;

#[derive(Properties, PartialEq)]
pub struct ReadmeProperties {
    pub project_id: ProjectId,
}

#[function_component]
pub fn ReadmeView(props: &ReadmeProperties) -> Html {
    let readme = PROJECTS[props.project_id].readme.to_string();
    let repo_html = match PROJECTS[props.project_id].repo {
        Some(link) => html! {
            <div class="repo-link">
                <a href={link} target="_blank" rel="noopener noreferrer">
                    {"See the code on GitHub"}
                </a>
            </div>
        },
        None => html! {},
    };
    html! {
        <Page>
            <div class="markdown-container">
                {repo_html}
                <MarkdownRenderer markdown={readme} />
            </div>
        </Page>
    }
}
