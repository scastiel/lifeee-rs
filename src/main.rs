use yew::prelude::*;

mod components;
mod life;

use components::counter::Counter;
use components::game::Game;
use components::title::Title;

#[function_component(App)]
fn app() -> Html {
  html! {
    <>
    <Title label="Hey Yew!"/>
    <hr/>
    <Counter/>
    <hr/>
    <Title label="Game"/>
    <Game/>
    </>
  }
}

fn main() {
  yew::start_app::<App>();
}
