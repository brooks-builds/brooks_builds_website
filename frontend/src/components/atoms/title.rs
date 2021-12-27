use stylist::Style;
use yew::prelude::*;

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub value: String,
    #[prop_or(false)]
    pub center: bool,
}

#[function_component(Title)]
pub fn title(props: &Props) -> Html {
    let style = Style::new(create_style()).unwrap();

    html! {
        <div class={ style.get_class_name().to_string() }>
            <style>{ style.get_style_str().to_string() }</style>
            <h1 class={ if props.center { "center" } else { "" } }>{ &props.value }</h1>
        </div>
    }
}

fn create_style() -> String {
    r#"
        .center {
            text-align: center;
        }
    "#
    .to_owned()
}
