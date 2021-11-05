use crate::life;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct BoardProps {
  pub cells: life::CellSet,
}

#[function_component(Board)]
pub fn board(BoardProps { cells }: &BoardProps) -> Html {
  html! {
    <ul>{
      cells
        .iter()
        .map(|&life::Cell { x, y }| {
          html! { <li>{x}{", "}{y}</li> }
        })
        .collect::<Html>()
    }</ul>
  }
}
