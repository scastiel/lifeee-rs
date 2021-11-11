use crate::components::board::Board;
use crate::components::pattern_selector::PatternSelector;
use crate::lexicon::Term;
use crate::life::*;
use crate::Settings;
use gloo::events::EventListener;
use gloo::timers::callback::Interval;
use std::collections::VecDeque;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub struct Game {
  cells: CellSet,
  previous_gens: Vec<CellSet>,
  tick: u32,
  interval: Option<Interval>,
  speed: u8,
  adjust_offset: Option<(usize, usize)>,
  offset: (f64, f64),
  zoom: f64,
  width: u32,
  height: u32,
  _resize_handle: EventListener,
}

pub enum Msg {
  NextTick,
  Play,
  Pause,
  ChangeSpeed(u8),
  ApplyPattern(Term),
  MoveOffset((f64, f64)),
  ChangeZoom((i32, i32, f64)),
  Resize,
}

impl Game {
  fn settings(&self, ctx: &Context<Self>) -> Settings {
    ctx
      .link()
      .context::<Settings>(Callback::noop())
      .expect("settings context to be set")
      .0
  }

  fn start_interval(&mut self, ctx: &Context<Self>) {
    let link = ctx.link().clone();
    link.send_message(Msg::NextTick);
    let millis = (50_f64 - 500_f64) / 9_f64 * self.speed as f64 + 500_f64;
    let interval = Interval::new(millis as u32, move || link.send_message(Msg::NextTick));
    self.interval = Some(interval);
  }
}

impl Component for Game {
  type Message = Msg;
  type Properties = ();

  fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    let settings = self.settings(ctx);
    match msg {
      Msg::NextTick => {
        self.tick += 1;
        self.adjust_offset = None;

        self.previous_gens = {
          let mut previous_gens_deque: VecDeque<CellSet> = self
            .previous_gens
            .iter()
            .map(|cell_set| cell_set.clone())
            .collect();
          previous_gens_deque.push_front(self.cells.clone());
          if previous_gens_deque.len() > settings.num_previous {
            previous_gens_deque.pop_back();
          }
          previous_gens_deque
            .iter()
            .map(|cell_set| cell_set.clone())
            .collect()
        };

        self.cells = tick(&self.cells);

        true
      }
      Msg::Play => {
        self.start_interval(ctx);
        true
      }
      Msg::Pause => {
        self.interval = None;
        true
      }
      Msg::ChangeSpeed(speed) => {
        self.speed = speed;
        if self.interval.is_some() {
          self.start_interval(ctx);
        }
        true
      }
      Msg::ApplyPattern(term) => {
        self.cells = term
          .cells
          .iter()
          .fold(CellSet::new(), |cells, &cell| make_cell_alive(&cells, cell));
        self.tick = 0;
        self.previous_gens = vec![];
        self.offset = (
          (self.width as f64 / 2_f64
            - term.width as f64 * self.zoom * (settings.cell_size + settings.grid_width) as f64
              / 2_f64),
          (self.height as f64 / 2_f64
            - term.height as f64 * self.zoom * (settings.cell_size + settings.grid_width) as f64
              / 2_f64),
        );
        true
      }
      Msg::MoveOffset(offset) => {
        self.offset = offset;
        true
      }
      Msg::Resize => {
        let window = web_sys::window().unwrap();
        let (width, height) = (
          window.inner_width().unwrap().as_f64().unwrap() as u32,
          window.inner_height().unwrap().as_f64().unwrap() as u32,
        );
        self.width = width;
        self.height = height;
        true
      }
      Msg::ChangeZoom((x1, y1, zoom)) => {
        let offset = self.offset;
        let prev_zoom = self.zoom;
        self.zoom = zoom;
        self.offset = (
          offset.0 - (x1 as f64 - offset.0) * (self.zoom / prev_zoom - 1.0),
          offset.1 - (y1 as f64 - offset.1) * (self.zoom / prev_zoom - 1.0),
        );
        true
      }
    }
  }

  fn create(ctx: &Context<Self>) -> Self {
    let window = web_sys::window().unwrap();
    let link = ctx.link().clone();
    let resize_handle = EventListener::new(&window, "resize", move |_: &Event| {
      link.send_message(Msg::Resize)
    });

    Self {
      cells: CellSet::new(),
      previous_gens: vec![] as Vec<CellSet>,
      tick: 0,
      interval: None,
      speed: 5,
      adjust_offset: None,
      offset: (0.0, 0.0),
      zoom: 1.0,
      width: 300,
      height: 200,
      _resize_handle: resize_handle,
    }
  }

  fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
    if _first_render {
      ctx.link().send_message(Msg::Resize);
    }
  }

  fn view(&self, ctx: &Context<Self>) -> yew::virtual_dom::VNode {
    let running = self.interval.is_some();

    let on_change_speed = ctx.link().callback(|event: Event| {
      let input = event
        .target()
        .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
        .unwrap();
      let speed: u8 = input.value().parse().unwrap();
      Msg::ChangeSpeed(speed)
    });

    let on_change_zoom = {
      let width = self.width;
      let height = self.height;
      ctx.link().callback(move |event: Event| {
        let input = event
          .target()
          .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
          .unwrap();
        let zoom: f64 = input.value().parse().unwrap();
        Msg::ChangeZoom((
          (width as f64 / 2_f64) as i32,
          (height as f64 / 2_f64) as i32,
          zoom,
        ))
      })
    };

    html! {
      <>
        <Board
          cells={self.cells.clone()}
          previous_gens={self.previous_gens.clone()}
          offset={self.offset}
          zoom={self.zoom}
          move_offset={ctx.link().callback(move |offset| Msg::MoveOffset(offset))}
          change_zoom={ctx.link().callback(move |(x1, y1, zoom)| Msg::ChangeZoom((x1, y1, zoom)))}
          width={self.width}
          height={self.height}
        />
        <div class="panel">
          <div class="controls">
            <button disabled={running} onclick={ctx.link().callback(|_| Msg::NextTick)}>{"Tick"}</button>
            <button onclick={
              if running {
                ctx.link().callback(|_| Msg::Pause)
              } else {
                ctx.link().callback(|_| Msg::Play)
              }
            }>{{if running { "Pause" } else { "Play" }}}</button>
            <span class="generation">{format!("Generation #{}", self.tick)}</span>
          </div>
          <PatternSelector on_apply_pattern={ctx.link().callback(|term| Msg::ApplyPattern(term))} />
          <label>
            <span>{"Speed"}</span>
            <input
              type="range" min="1" max="10"
              value={self.speed.to_string()}
              onchange={on_change_speed}
            />
          </label>
          <label>
            <span>{"Zoom"}</span>
            <input
              type="range" min="0.1" max="5.0" step="0.1"
              value={self.zoom.to_string()}
              onchange={on_change_zoom}
            />
          </label>
        </div>
      </>
    }
  }
}
