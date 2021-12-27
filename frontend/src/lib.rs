use app::App;

mod app;
mod components;
mod router;
mod views;

pub fn run() {
    yew::start_app::<App>();
}
