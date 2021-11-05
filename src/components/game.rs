use crate::components::board::Board;
use crate::life;
use yew::prelude::*;

#[function_component(Game)]
pub fn game() -> Html {
  let cells = use_state(|| {
    let alive_cells = vec![
      life::Cell { x: 0, y: 0 },
      life::Cell { x: 0, y: 1 },
      life::Cell { x: 0, y: 2 },
    ];
    alive_cells
      .iter()
      .fold(life::CellSet::new(), |cells, &cell| {
        life::make_cell_alive(&cells, cell)
      })
  });
  let tick = use_state(|| 0);

  let on_click_next_gen = {
    let tick = tick.clone();
    Callback::from(move |_| {
      tick.set(*tick + 1);
    })
  };

  {
    let cells = cells.clone();
    use_effect_with_deps(
      move |&tick| {
        if tick > 0 {
          cells.set(life::tick(&cells));
        }
        || {}
      },
      *tick,
    );
  }

  html! {
    <>
      <Board cells={
          cells
            .iter()
            .map(|cell| cell.clone())
            .collect::<life::CellSet>()
        } />
      <button onclick={on_click_next_gen}>{"Tick"}</button>
      <p>{"Generation #"}{*tick}</p>
    </>
  }
}
