#![allow(non_camel_case_types)]
use yew::prelude::*;

use crate::app::components::markdown::MarkdownRenderer;
use crate::app::components::page::Page;
use crate::app::views::main::MainView;
use crate::data::PROJECTS_MAP;

#[derive(Properties, PartialEq)]
pub struct ReadmeProperties {
    pub project_path: String,
}

#[function_component]
pub fn ReadmeView(props: &ReadmeProperties) -> Html {
    let project = PROJECTS_MAP.get(&props.project_path);
    let project = match project {
        Some(readme) => readme,
        None => {
            return html! {
                <MainView />
            }
        },
    };
    let readme = project.readme.to_string();
    let repo_html = match project.repo {
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
