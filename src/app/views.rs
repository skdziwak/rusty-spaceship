use yew_router::prelude::*;

pub mod main;
pub mod contact;
pub mod about;
pub mod projects;
pub mod readme;

#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    #[not_found]
    Main,
    #[at("/about")]
    About,
    #[at("/projects")]
    Projects,
    #[at("/contact")]
    Contact,
    #[at("/projects/:id")]
    Readme { id: ProjectId },
}

type ProjectId = usize;


