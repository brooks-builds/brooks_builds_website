use yew::prelude::*;

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub value: String,
}

#[function_component(Pill)]
pub fn pill(props: &Props) -> Html {
    html! {
        <div>{ &props.value }</div>
    }
}
