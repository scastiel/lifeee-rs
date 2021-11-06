pub fn grey(coeff: f64) -> String {
  let coeff = f64::min(f64::max(coeff, 0.0), 1.0);
  let v = (coeff * 255.0) as u8;
  format!("#{:0>2x}{:0>2x}{:0>2x}", v, v, v)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn grey_minus_0_5_returns_black() {
    assert_eq!(grey(-0.5), "#000000".to_string())
  }

  #[test]
  fn grey_0_returns_black() {
    assert_eq!(grey(0.0), "#000000".to_string())
  }

  #[test]
  fn grey_0_5_returns_some_grey() {
    assert_eq!(grey(0.5), "#7f7f7f".to_string())
  }

  #[test]
  fn grey_1_returns_white() {
    assert_eq!(grey(1.0), "#ffffff".to_string())
  }

  #[test]
  fn grey_1_5_returns_white() {
    assert_eq!(grey(1.5), "#ffffff".to_string())
  }
}
