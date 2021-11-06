pub struct Settings {
  pub cell_size: f64,
  pub grid_width: f64,
  pub num_previous: usize,
}

pub fn default_settings() -> Settings {
  Settings {
    cell_size: 20.0,
    grid_width: 1.0,
    num_previous: 1,
  }
}
