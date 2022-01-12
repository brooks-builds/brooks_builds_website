use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
      <div>
        <h1 data-test-hello="true">{"Hello World"}</h1>
      </div>
    }
}
