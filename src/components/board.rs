use crate::life;
use crate::settings::*;
use wasm_bindgen::*;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct BoardProps {
  pub cells: life::CellSet,
  pub previous_gens: Vec<life::CellSet>,
}

pub struct Board {
  canvas_ref: NodeRef,
  offset: (i32, i32),
  last_offset: Option<(i32, i32)>,
}

impl Board {
  fn canvas(&self) -> web_sys::HtmlCanvasElement {
    self
      .canvas_ref
      .cast::<web_sys::HtmlCanvasElement>()
      .unwrap()
  }

  fn context(&self) -> web_sys::CanvasRenderingContext2d {
    self
      .canvas()
      .get_context("2d")
      .unwrap()
      .unwrap()
      .dyn_into::<web_sys::CanvasRenderingContext2d>()
      .unwrap()
  }

  fn cell_range(&self) -> (std::ops::Range<i32>, std::ops::Range<i32>) {
    let canvas = self.canvas();

    let from_x = self.size_to_cells(-self.offset.0 as f64) - 1;
    let to_x = self.size_to_cells(canvas.width() as f64 - self.offset.0 as f64);
    let from_y = self.size_to_cells(-self.offset.1 as f64 - 1_f64) - 1;
    let to_y = self.size_to_cells(canvas.height() as f64 - self.offset.1 as f64);

    (from_x..to_x, from_y..to_y)
  }

  fn erase(&self) {
    let canvas = self.canvas();
    let context = self.context();
    context.set_fill_style(&JsValue::from_str("white"));
    context.fill_rect(0.0, 0.0, canvas.width().into(), canvas.height().into())
  }

  fn size_to_cells(&self, size: f64) -> i32 {
    let settings = default_settings();
    (size / (settings.cell_size + settings.grid_width) as f64).ceil() as i32
  }

  fn draw_grid(&self) {
    let settings = default_settings();
    let canvas = self.canvas();
    let context = self.context();
    context.set_fill_style(&JsValue::from_str("lightgray"));

    let (cell_range_x, cell_range_y) = self.cell_range();
    for i in cell_range_x {
      context.fill_rect(
        self.offset.0 as f64 + i as f64 * (settings.cell_size + settings.grid_width),
        0.0,
        settings.grid_width,
        canvas.height().into(),
      )
    }

    for j in cell_range_y {
      context.fill_rect(
        0.0,
        self.offset.1 as f64 + j as f64 * (settings.cell_size + settings.grid_width),
        canvas.width().into(),
        settings.grid_width,
      )
    }
  }

  fn draw_cells(&self, cells: &life::CellSet, color: String) {
    let settings = default_settings();
    let context = self.context();
    context.set_fill_style(&JsValue::from(color));

    let (cell_range_x, cell_range_y) = self.cell_range();
    let cells = cells.iter().filter(|life::Cell { x, y }| {
      *x >= cell_range_x.start
        && *x <= cell_range_x.end
        && *y >= cell_range_y.start
        && *y <= cell_range_y.end
    });

    for cell in cells {
      context.fill_rect(
        self.offset.0 as f64
          + settings.grid_width
          + (settings.cell_size + settings.grid_width) * cell.x as f64,
        self.offset.1 as f64
          + settings.grid_width
          + (settings.cell_size + settings.grid_width) * cell.y as f64,
        settings.cell_size,
        settings.cell_size,
      );
    }
  }

  fn color_for_previous_gen(&self, gen_index: usize, num_gens: usize) -> String {
    let from = 0.7_f64;
    let to = 0.95_f64;
    let coeff = gen_index as f64 * (to - from) / (num_gens as f64) + from;
    crate::color_utils::grey(coeff)
  }
}

pub enum BoardMessage {
  PointerDown(i32, i32),
  PointerUp(i32, i32),
  PointerMove(i32, i32),
}

impl Component for Board {
  type Message = BoardMessage;
  type Properties = BoardProps;

  fn create(_ctx: &Context<Self>) -> Self {
    Self {
      canvas_ref: NodeRef::default(),
      offset: (0, 0),
      last_offset: None,
    }
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      BoardMessage::PointerDown(x, y) => {
        self.last_offset = Some((x, y));
        false
      }
      BoardMessage::PointerUp(_x, _y) => {
        self.last_offset = None;
        false
      }
      BoardMessage::PointerMove(x, y) => {
        if let Some(last_offset) = self.last_offset {
          self.offset = (
            self.offset.0 + x - last_offset.0,
            self.offset.1 + y - last_offset.1,
          );
          self.last_offset = Some((x, y));
          true
        } else {
          false
        }
      }
    }
  }

  fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
    self.erase();
    self.draw_grid();
    let previous_gens = &ctx.props().previous_gens;
    let num_gens = previous_gens.len();
    for i in 0..num_gens {
      let gen_index = num_gens - i - 1;
      self.draw_cells(
        &previous_gens[gen_index],
        self.color_for_previous_gen(gen_index, num_gens),
      );
    }
    self.draw_cells(&ctx.props().cells, "black".to_string());
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <>
        <canvas
          ref={self.canvas_ref.clone()} style="border: 1px solid lightgray; cursor: move"
          width={300}
          height={200}
          onpointerdown={ctx.link().callback(|event: PointerEvent| BoardMessage::PointerDown(event.client_x(), event.client_y()))}
          onpointerup={ctx.link().callback(|event: PointerEvent| BoardMessage::PointerUp(event.client_x(), event.client_y()))}
          onpointerout={ctx.link().callback(|event: PointerEvent| BoardMessage::PointerUp(event.client_x(), event.client_y()))}
          onpointermove={ctx.link().callback(|event: PointerEvent| BoardMessage::PointerMove(event.client_x(), event.client_y()))}
          />
        // <ul>{
        //   ctx.props().cells
        //     .iter()
        //     .map(|&life::Cell { x, y }| {
        //       html! { <li>{x}{", "}{y}</li> }
        //     })
        //     .collect::<Html>()
        // }</ul>
      </>
    }
  }
}
