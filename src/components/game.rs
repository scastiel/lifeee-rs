use crate::components::board::Board;
use crate::life;
use std::collections::VecDeque;
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
  let previous_gens = use_state(|| vec![] as Vec<life::CellSet>);
  let tick = use_state(|| 0);

  let on_click_next_gen = {
    let tick = tick.clone();
    Callback::from(move |_| {
      tick.set(*tick + 1);
    })
  };

  {
    let cells = cells.clone();
    let previous_gens = previous_gens.clone();

    let mut previous_gens_deque: VecDeque<life::CellSet> = previous_gens
      .iter()
      .map(|cell_set| cell_set.clone())
      .collect();
    previous_gens_deque.push_front(
      cells
        .iter()
        .map(|cell| cell.clone())
        .collect::<life::CellSet>(),
    );
    if previous_gens_deque.len() > 1 {
      previous_gens_deque.pop_back();
    }

    use_effect_with_deps(
      move |&tick| {
        if tick > 0 {
          cells.set(life::tick(&cells));
          previous_gens.set(
            previous_gens_deque
              .iter()
              .map(|cell_set| cell_set.clone())
              .collect(),
          );
        }
        || {}
      },
      *tick,
    );
  }

  html! {
    <>
      <div>
        <Board
          cells={
            cells.iter()
              .map(|cell| cell.clone())
              .collect::<life::CellSet>()
          }
          previous_gens={
            previous_gens.iter()
              .map(|cell_set| cell_set.clone())
              .collect::<Vec<life::CellSet>>()
          }
        />
      </div>
      <button onclick={on_click_next_gen}>{"Tick"}</button>
      <p>{"Generation #"}{*tick}</p>
    </>
  }
}
