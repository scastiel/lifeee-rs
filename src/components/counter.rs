use gloo_console::log;
use yew::prelude::*;

#[function_component(Counter)]
pub fn counter() -> Html {
  let counter = use_state(|| 0);
  let on_click = {
    let counter = counter.clone();
    Callback::from(move |_| counter.set(*counter + 1))
  };

  use_effect_with_deps(
    |(counter1, _)| {
      log!(format!("Counter = {}", *counter1));
      || {}
    },
    (*counter, *counter),
  );

  html! {
    <>
      <p>{"Value: "}{*counter}</p>
      <button onclick={on_click}>{"Increment"}</button>
    </>
  }
}
