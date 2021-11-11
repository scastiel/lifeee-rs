use crate::color_utils::grey;
use crate::life;
use crate::settings::Settings;
use wasm_bindgen::*;
use web_sys::WheelEvent;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct BoardProps {
  pub cells: life::CellSet,
  pub previous_gens: Vec<life::CellSet>,
  pub offset: (f64, f64),
  pub zoom: f64,
  pub move_offset: Callback<(f64, f64)>,
  pub change_zoom: Callback<(i32, i32, f64)>,
  pub width: u32,
  pub height: u32,
}

pub struct Board {
  canvas_ref: NodeRef,
  last_offset: Option<(f64, f64)>,
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

  fn cell_range(
    &self,
    settings: &Settings,
    offset: (f64, f64),
    zoom: f64,
  ) -> (std::ops::Range<i32>, std::ops::Range<i32>) {
    let canvas = self.canvas();

    let from_x = self.size_to_cells(settings, -offset.0, zoom) as i32 - 1;
    let to_x = from_x + self.size_to_cells(settings, canvas.width() as f64, zoom) as i32 + 1;
    let from_y = self.size_to_cells(settings, -offset.1, zoom) as i32 - 1;
    let to_y = from_y + self.size_to_cells(settings, canvas.height() as f64, zoom) as i32 + 1;

    (from_x..to_x, from_y..to_y)
  }

  fn erase(&self) {
    let canvas = self.canvas();
    let context = self.context();
    context.set_fill_style(&JsValue::from_str("white"));
    context.fill_rect(0.0, 0.0, canvas.width().into(), canvas.height().into())
  }

  fn size_to_cells(&self, settings: &Settings, size: f64, zoom: f64) -> f64 {
    (size / (settings.cell_size * zoom + settings.grid_width) as f64).ceil()
  }

  fn draw_grid(&self, settings: &Settings, offset: (f64, f64), zoom: f64) {
    let canvas = self.canvas();
    let context = self.context();
    context.set_fill_style(&JsValue::from_str(grey(0.9).as_str()));

    let (cell_range_x, cell_range_y) = self.cell_range(settings, offset, zoom);
    for i in cell_range_x {
      context.fill_rect(
        offset.0 + i as f64 * (zoom * settings.cell_size + settings.grid_width),
        0.0,
        settings.grid_width,
        canvas.height().into(),
      )
    }

    for j in cell_range_y {
      context.fill_rect(
        0.0,
        offset.1 + (j as f64 * (zoom * settings.cell_size + settings.grid_width)),
        canvas.width().into(),
        settings.grid_width,
      )
    }
  }

  fn draw_cells(
    &self,
    settings: &Settings,
    cells: &life::CellSet,
    color: String,
    offset: (f64, f64),
    zoom: f64,
  ) {
    let context = self.context();
    context.set_fill_style(&JsValue::from(color));

    let (cell_range_x, cell_range_y) = self.cell_range(settings, offset, zoom);
    let cells = cells.iter().filter(|life::Cell { x, y }| {
      *x >= cell_range_x.start
        && *x <= cell_range_x.end
        && *y >= cell_range_y.start
        && *y <= cell_range_y.end
    });

    for cell in cells {
      context.fill_rect(
        offset.0
          + (settings.grid_width
            + (zoom * settings.cell_size + settings.grid_width) * cell.x as f64),
        offset.1
          + (settings.grid_width
            + (zoom * settings.cell_size + settings.grid_width) * cell.y as f64),
        zoom * settings.cell_size,
        zoom * settings.cell_size,
      );
    }
  }

  fn color_for_previous_gen(&self, gen_index: usize, num_gens: usize) -> String {
    let from = 0.80_f64;
    let to = 0.99_f64;
    let coeff = gen_index as f64 * (to - from) / (num_gens as f64) + from;
    crate::color_utils::grey(coeff)
  }

  fn settings(&self, ctx: &Context<Self>) -> Settings {
    ctx
      .link()
      .context::<Settings>(Callback::noop())
      .expect("settings context to be set")
      .0
  }
}

pub enum BoardMessage {
  PointerDown(i32, i32),
  PointerUp(i32, i32),
  PointerMove(i32, i32),
  Zoom(i32, i32, f64),
}

impl Component for Board {
  type Message = BoardMessage;
  type Properties = BoardProps;

  fn create(_ctx: &Context<Self>) -> Self {
    Self {
      canvas_ref: NodeRef::default(),
      last_offset: None,
    }
  }

  fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      BoardMessage::PointerDown(x, y) => {
        self.last_offset = Some((x as f64, y as f64));
        false
      }
      BoardMessage::PointerUp(_x, _y) => {
        self.last_offset = None;
        false
      }
      BoardMessage::PointerMove(x, y) => {
        if let Some(last_offset) = self.last_offset {
          let offset = ctx.props().offset;
          let new_offset = (
            offset.0 + x as f64 - last_offset.0,
            offset.1 + y as f64 - last_offset.1,
          );
          if offset != new_offset {
            ctx.props().move_offset.emit((
              offset.0 + x as f64 - last_offset.0,
              offset.1 + y as f64 - last_offset.1,
            ));
          }
          self.last_offset = Some((x as f64, y as f64));
          true
        } else {
          false
        }
      }
      BoardMessage::Zoom(x1, y1, zoom) => {
        let zoom = f64::max(f64::min(ctx.props().zoom - 0.1 * zoom / 120.0, 5.0), 0.1);
        ctx.props().change_zoom.emit((x1, y1, zoom));
        true
      }
    }
  }

  fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
    let canvas = self.canvas();
    canvas.set_width(ctx.props().width);
    canvas.set_height(ctx.props().height);
    let settings = self.settings(ctx);
    let zoom = ctx.props().zoom;
    let offset = ctx.props().offset;
    self.erase();
    if ctx.props().zoom > 0.3 {
      self.draw_grid(&settings, offset, zoom);
    }
    let previous_gens = &ctx.props().previous_gens;
    let num_gens = previous_gens.len();
    for i in 0..num_gens {
      let gen_index = num_gens - i - 1;
      self.draw_cells(
        &settings,
        &previous_gens[gen_index],
        self.color_for_previous_gen(gen_index, num_gens),
        offset,
        zoom,
      );
    }
    self.draw_cells(
      &settings,
      &ctx.props().cells,
      "black".to_string(),
      offset,
      zoom,
    );
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <canvas
        ref={self.canvas_ref.clone()}
        style="cursor: move; position: absolute; top: 0; right: 0; bottom: 0; left: 0"
        width={ctx.props().width.to_string()}
        height={ctx.props().height.to_string()}
        onpointerdown={ctx.link().callback(|event: PointerEvent| BoardMessage::PointerDown(event.client_x(), event.client_y()))}
        onpointerup={ctx.link().callback(|event: PointerEvent| BoardMessage::PointerUp(event.client_x(), event.client_y()))}
        onpointerout={ctx.link().callback(|event: PointerEvent| BoardMessage::PointerUp(event.client_x(), event.client_y()))}
        onpointermove={ctx.link().callback(|event: PointerEvent| BoardMessage::PointerMove(event.client_x(), event.client_y()))}
        onwheel={ctx.link().callback(|event: WheelEvent| BoardMessage::Zoom(event.client_x(), event.client_y(), event.delta_y()))}
      />
    }
  }
}
