use lexicon::Cell;
use std::collections::HashSet;

pub type CellSet = HashSet<Cell>;

fn singleton(cell: Cell) -> CellSet {
  let mut cells = CellSet::new();
  cells.insert(cell);
  cells
}

pub fn make_cell_alive(cells: &CellSet, cell: Cell) -> CellSet {
  cells.union(&singleton(cell)).copied().collect()
}

pub fn make_cell_dead(cells: &CellSet, cell: Cell) -> CellSet {
  cells.difference(&singleton(cell)).copied().collect()
}

pub fn tick(cells: &CellSet) -> CellSet {
  cells_with_neighbors(cells)
    .iter()
    .filter(|&&cell| {
      let alive_neighbors = number_of_alive_neighbors(cells, cell);
      if cell_is_alive(cells, cell) {
        alive_neighbors == 2 || alive_neighbors == 3
      } else {
        alive_neighbors == 3
      }
    })
    .map(|&c| c)
    .collect()
}

pub fn cell_is_alive(cells: &CellSet, cell: Cell) -> bool {
  cells.contains(&cell)
}

fn cells_with_neighbors(cells: &HashSet<Cell>) -> CellSet {
  cells
    .iter()
    .flat_map(|&cell| {
      let mut neighbors = cell_neighbors(cell);
      neighbors.push(cell);
      neighbors
    })
    .collect()
}

fn number_of_alive_neighbors(cells: &CellSet, cell: Cell) -> usize {
  cell_neighbors(cell)
    .iter()
    .map(|&neighbor| cell_is_alive(cells, neighbor))
    .filter(|&b| b)
    .count()
}

fn cell_neighbors(cell: Cell) -> Vec<Cell> {
  (-1..=1)
    .into_iter()
    .flat_map(|dx| {
      (-1..=1).into_iter().flat_map(move |dy| {
        let mut set = vec![];
        if dx != 0 || dy != 0 {
          set.push(Cell {
            x: cell.x + dx,
            y: cell.y + dy,
          });
        }
        set
      })
    })
    .collect()
}
