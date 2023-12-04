use std::collections::HashSet;

advent_of_code::solution!(3);

#[derive(Debug, PartialEq)]
struct Position(usize, usize);

#[derive(Debug)]
struct Cell {
    value: char,
    position: Position,
}

fn get_symbols(map: &Vec<Vec<char>>) -> Vec<char> {
    let mut symbols: HashSet<char> = HashSet::new();
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            // if not digit, and not . add to vec
            if !map[i][j].is_digit(10) && map[i][j] != '.' {
                symbols.insert(map[i][j]);
            }
        }
    }
    symbols.into_iter().collect()
}

// dfs immediate neighbours for digits
fn find_neighbours(map: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<Cell> {
    let mut neighbours: Vec<Cell> = Vec::new();

    for row in x.checked_sub(1).unwrap_or(0)..=x + 1 {
        for col in y.checked_sub(1).unwrap_or(0)..=y + 1 {
            if row == x && col == y || row >= map.len() || col >= map[row].len() {
                continue;
            }
            neighbours.push(Cell {
                value: map[row][col],
                position: Position(row, col),
            });
        }
    }

    neighbours
}

// linear search for digits that make up number
fn find_number(map: &Vec<Vec<char>>, Cell { position: Position(x, y), ..}: Cell) -> (u32, Vec<Position>) {
    let mut right = y;
    let mut left = y;

    // double sliding window to find entire number
    while left > 0 && map[x][left - 1].is_digit(10) {
        left -= 1;
    }
    while right < map[x].len() - 1 && map[x][right + 1].is_digit(10) {
        right += 1;
    }

    let number: u32 = map[x][left..=right]
        .iter()
        .collect::<String>()
        .parse()
        .unwrap();

    // collect visited cells to avoid duplicates
    let visited_cells: Vec<Position> = (left..=right)
        .map(|i| Position(x, i))
        .collect::<Vec<Position>>();

    (number, visited_cells)
}

fn search_for_parts(map: &Vec<Vec<char>>, symbols: &Vec<char>, process: &dyn Fn(&Vec<Vec<char>>, Vec<Cell>) -> (u32, Vec<Position>)) -> u32 {
    println!("searching for symbols: {:?}", symbols);

    let mut parts: Vec<u32> = Vec::new();
    let mut visited_cells: Vec<Position> = Vec::new();

    // search each cell, check if symbol
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            let char = map[row][col];

            if symbols.contains(&char) {
                let neighbours = find_neighbours(&map, row, col);

                let (number, cells) = process(map, neighbours);
                parts.push(number);
                visited_cells.extend(cells);
            }
        }
    }

    println!("parts: {:?}", parts);
    parts.iter().sum::<u32>().into()
}

pub fn part_one(input: &str) -> Option<u32> {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let symbols: Vec<char> = get_symbols(&map);

    fn process(map: &Vec<Vec<char>>, neighbours: Vec<Cell>) -> (u32, Vec<Position>) {
        let mut valid_neighbours: Vec<u32> = Vec::new();
        let mut visited_cells: Vec<Position> = Vec::new();

        for neighbour in neighbours {
            if neighbour.value.is_digit(10) && !visited_cells.contains(&neighbour.position) {
                let (number, cells) = find_number(map, neighbour);
                valid_neighbours.push(number);
                visited_cells.extend(cells);
            };
        }

        (valid_neighbours.iter().sum::<u32>(), visited_cells)
    }

    Some(search_for_parts(&map, &symbols, &process))
}

pub fn part_two(input: &str) -> Option<u32> {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let symbols: Vec<char> = vec!['*'];

    fn process(map: &Vec<Vec<char>>, neighbours: Vec<Cell>) -> (u32, Vec<Position>) {
        let mut gears: u32 = 0;
        let mut valid_neighbours: Vec<u32> = Vec::new();
        let mut visited_cells: Vec<Position> = Vec::new();

        for neighbour in neighbours {
            if neighbour.value.is_digit(10) && !visited_cells.contains(&neighbour.position) {
                let (number, cells) = find_number(map, neighbour);
                valid_neighbours.push(number);
                visited_cells.extend(cells);
            };
        }

        if valid_neighbours.len() == 2 {
            gears = valid_neighbours[0] * valid_neighbours[1];
        }

        (gears, visited_cells)
    }

    Some(search_for_parts(&map, &symbols, &process))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
