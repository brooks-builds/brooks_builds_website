use super::router::{switch, Routes};
use stylist::Style;
use yew::prelude::*;
use yew_router::prelude::*;

pub struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        let style = Style::new(create_style()).expect("Error creating style string for app");

        html! {
            <>
            <style>{ style.get_style_str().to_owned() }</style>
                <div class={ style.get_class_name().to_owned() }>
                    <div class={ "app" }>
                        <BrowserRouter>
                            <Switch<Routes> render={ Switch::render(switch) } />
                        </BrowserRouter>
                    </div>
                </div>
            </>
        }
    }
}

fn create_style() -> String {
    r#"
        .app {
            margin: 1rem;
        }
    "#
    .to_string()
}
