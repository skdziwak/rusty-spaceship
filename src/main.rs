mod app;
mod data;
mod schema;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
