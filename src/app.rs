use yew_router::prelude::*;
use yew::prelude::*;

use views::{Route, main::MainView, contact::ContactView, about::AboutView, projects::ProjectsView, readme::ReadmeView};

pub mod views;
pub mod components;

fn switch(state: Route) -> Html {
    match state {
        Route::Main => html! { <MainView /> },
        Route::About => html! { <AboutView /> },
        Route::Projects => html! { <ProjectsView /> },
        Route::Contact => html! { <ContactView /> },
        Route::Readme { path } => html! { <ReadmeView project_path={path.clone()} /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </main>
    }
}
