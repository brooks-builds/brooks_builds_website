use super::super::components::atoms::title::Title;
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div>
            <section class={ "title" }>
                <Title value={ "Brooks Builds" } center={ true } />
            </section>
        </div>
    }
}
