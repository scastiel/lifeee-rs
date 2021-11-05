use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TitleProps {
  pub label: String,
}

#[function_component(Title)]
pub fn title(TitleProps { label }: &TitleProps) -> Html {
  html! {
    <h1>{label}</h1>
  }
}
