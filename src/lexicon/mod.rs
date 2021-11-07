use regex::Regex;
use std::fmt::Debug;

#[derive(Debug)]
pub struct ParseError(String);

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct Cell {
  pub x: i32,
  pub y: i32,
}

#[derive(Debug, Clone)]
pub struct Term {
  pub name: String,
  pub cells: Vec<Cell>,
}

#[derive(Debug)]
pub struct Lexicon {
  pub terms: Vec<Term>,
}

impl Lexicon {
  pub fn get_term(&self, name: String) -> Option<&Term> {
    self.terms.iter().find(|term| term.name == name)
  }
}

fn read_term_name(line: String) -> Option<String> {
  let re = Regex::new(r"^:(?P<name>.+):").unwrap();
  re.captures(line.as_str())
    .and_then(|cap| cap.name("name").map(|name| name.as_str().to_string()))
}

fn read_cells_line(line: String, y: i32) -> Vec<Cell> {
  let line = line.strip_prefix("\t").unwrap();
  let mut cells = vec![];
  for (x, c) in line.char_indices() {
    if c == '*' {
      cells.push(Cell { x: x as i32, y });
    }
  }
  cells
}

pub fn get_lexicon() -> Result<Lexicon, ParseError> {
  let mut lines = include_str!("lexicon.txt").lines();

  loop {
    match lines.next() {
      Some(line) => {
        if line.to_string().starts_with("----") {
          break;
        }
      }
      _ => return Err(ParseError("Unexpected end of input".to_string())),
    }
  }

  let mut lexicon = Lexicon { terms: vec![] };

  loop {
    match lines.next() {
      Some(line) => {
        let line = line.to_string();
        if line.starts_with("----") {
          break;
        }
        if line.starts_with(":") {
          if let Some(name) = read_term_name(line) {
            let mut current_term = Term {
              name,
              cells: vec![],
            };

            let mut cells_line = 0;
            loop {
              match lines.next() {
                Some(line) => {
                  let line = line.to_string();
                  if line.is_empty() {
                    break;
                  }
                  if line.starts_with("\t") {
                    let mut cells = read_cells_line(line, cells_line);
                    current_term.cells.append(&mut cells);
                    cells_line += 1;
                  }
                }
                _ => return Err(ParseError("Unexpected end of input".to_string())),
              }
            }

            lexicon.terms.push(current_term);
          } else {
            return Err(ParseError("Can't parse term name.".to_string()));
          }
        }
      }
      _ => return Err(ParseError("Unexpected end of input".to_string())),
    }
  }

  Ok(lexicon)
}
