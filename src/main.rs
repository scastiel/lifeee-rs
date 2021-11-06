use yew::prelude::*;

mod color_utils;
mod components;
mod life;
mod settings;

use components::counter::Counter;
use components::game::Game;
use components::title::Title;
use settings::{default_settings, Settings};

#[function_component(App)]
fn app() -> Html {
  html! {
    <>
    <Title label="Hey Yew!"/>
    <hr/>
    <Counter/>
    <hr/>
    <Title label="Game"/>
    <ContextProvider<Settings> context={default_settings()}>
      <Game/>
    </ContextProvider<Settings>>
    </>
  }
}

fn main() {
  yew::start_app::<App>();
}
