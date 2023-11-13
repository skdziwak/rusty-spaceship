use yew::prelude::*;
use crate::app::components::html::HtmlRenderer;
use crate::app::components::page::Page;
use crate::data::ABOUT;

#[function_component]
pub fn AboutView() -> Html {
    let skills: Vec<Html> = ABOUT.skills.iter().map(|skill| {
        html! {
            <div class="skill">
                <h3>{&skill.name}</h3>
                <p>{&skill.content}</p>
            </div>
        }
    }).collect();
    let tech: Vec<Html> = ABOUT.technologies.iter().map(|tech| {
        html! {
            <div class="skill">
                <h3>{&tech.name}</h3>
                <p>{&tech.content}</p>
            </div>
        }
    }).collect();
    let about_me = ABOUT.about_me.to_string();
    html! {
        <Page>
            <div class="content-container">
                <h1>{"About Me"}</h1>
                <section class="intro-section">
                    <HtmlRenderer html={about_me}/>
                </section>

                <section class="skills-grid-section">
                    <h2>{"Key Skills"}</h2>
                    <div class="skills-grid">
                        {skills}
                    </div>
                </section>

                <section class="skills-grid-section">
                    <h2>{"Technologies I use"}</h2>
                    <div class="skills-grid">
                        {tech}
                    </div>
                </section>

                <section class="cta-section">
                    <a href="contact" class="cta-button">{"Get In Touch"}</a>
                </section>
            </div>
        </Page>
    }
}
