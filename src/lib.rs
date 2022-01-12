use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
      <div>
        <h1 data-test-hello="true">{"Hello World"}</h1>
      </div>
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn canary() {
        let five = 5;
        assert_eq!(five, 5);
    }
}
