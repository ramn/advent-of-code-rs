use std::fmt;

const INPUT: &'static str = include_str!("resources/day_08_input.txt");
const ON: char = '#';

struct Grid {
    rows: usize,
    cols: usize,
    data: Vec<char>,
}

impl Grid {
    fn new() -> Grid {
        Grid {
            rows: 6,
            cols: 50,
            data: vec![' '; 50 * 6],
        }
    }

    fn get_col(&self, col: usize) -> Vec<char> {
        self.get_col_indices(col).iter().map(|&ix| self.data[ix]).collect()
    }

    fn get_row(&self, row: usize) -> Vec<char> {
        self.get_row_indices(row).iter().map(|&ix| self.data[ix]).collect()
    }

    fn write_row<I: IntoIterator<Item=char>>(&mut self, row: usize, xs: I) {
        let indices = self.get_row_indices(row);
        for (ix, new_value) in indices.into_iter().zip(xs.into_iter()) {
            self.data[ix] = new_value;
        }
    }

    fn write_col<I>(
        &mut self, col: usize, xs: I
    ) where I: IntoIterator<Item=char> {
        let indices = self.get_col_indices(col);
        for (ix, new_value) in indices.into_iter().zip(xs.into_iter()) {
            self.data[ix] = new_value;
        }
    }

    fn write_area(&mut self, width: usize, height: usize) {
        for row in 0..height {
            for col in 0..width {
                let ix = self.ix_for(row, col);
                self.data[ix] = ON;
            }
        }
    }

    fn rotate_row(&mut self, row: usize, by: usize) {
        let to_write = self.get_row(row).into_iter().cycle()
            .skip(self.cols - (by % self.cols));
        self.write_row(row, to_write);
    }

    fn rotate_col(&mut self, col: usize, by: usize) {
        let to_write = self.get_col(col).into_iter().cycle()
            .skip(self.rows - (by % self.rows));
        self.write_col(col, to_write);
    }

    fn count_lit_pixels(&self) -> usize {
        self.data.iter().filter(|&&c| c == ON).count()
    }

    fn get_row_indices(&self, row: usize) -> Vec<usize> {
        (0..self.cols).map(|col| self.ix_for(row, col)).collect()
    }

    fn get_col_indices(&self, col: usize) -> Vec<usize> {
        (0..self.rows).map(|row| self.ix_for(row, col)).collect()
    }

    fn ix_for(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }
}

pub fn problem_1() -> String {
    let mut grid = Grid::new();
    run(&mut grid);
    grid.count_lit_pixels().to_string()
}

pub fn problem_2() -> String {
    let mut grid = Grid::new();
    run(&mut grid);
    println!("\n{}", grid);
    format!("{}", grid)
}

fn run(grid: &mut Grid) {
    for line in INPUT.lines() {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        match tokens[0] {
            "rect" => {
                let mut size = tokens[1].split("x");
                let width = size.next().unwrap().parse().unwrap();
                let height = size.next().unwrap().parse().unwrap();
                grid.write_area(width, height);
            }
            "rotate" => {
                let ix = tokens[2][2..].parse().unwrap();
                let amount = tokens[4].parse().unwrap();
                match tokens[1] {
                    "row" => {
                        grid.rotate_row(ix, amount);
                    }
                    "column" => {
                        grid.rotate_col(ix, amount);
                    }
                    _ => unreachable!()
                }
            }
            _ => unreachable!()
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (0..self.rows).map(|row_ix| {
            let s: String = self.get_row(row_ix).into_iter().collect();
            writeln!(f, "{}", s)
        }).last().unwrap()
    }
}


#[cfg(test)]
mod tests {
    use std::time::Instant;
    use super::*;

    #[test]
    fn test_problem_1() {
        let t = Instant::now();
        let answer = problem_1();
        println!("Test took: {:?}", t.elapsed());
        assert_eq!("106", &answer);
    }

    #[test]
    fn test_problem_2() {
        // CFLELOYFCS
        let expected = include_str!("resources/day_08_part_2_expected.txt");
        let answer = problem_2();
        assert_eq!(expected, &answer);
    }
}
