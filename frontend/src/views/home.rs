use super::super::components::atoms::pill::Pill;
use super::super::components::atoms::title::Title;
use stylist::Style;
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    let style = Style::new(create_style()).expect("error creating style for home");

    html! {
        <div class={ style.get_class_name().to_owned() }>
            <style>{ style.get_style_str().to_owned() }</style>
            <section class={ "title" }>
                <Title value={ "Brooks Builds" } center={ true } />
                <div class={ "subtitle" }>
                    <Pill value={ "Coding" } />
                </div>
            </section>
        </div>
    }
}

fn create_style() -> String {
    r#"
        .subtitle {
            display: flex;
            justify-content: space-evenly;
        }
    "#
    .to_owned()
}
