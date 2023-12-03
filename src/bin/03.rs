use itertools::PeekingNext;
advent_of_code::solution!(3);

fn get_grid(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.to_owned().into_bytes())
        .collect::<Vec<Vec<u8>>>()
}

fn is_special_char(c: u8) -> bool {
    // ascii special characters are between 33 and 63, not including the dot (46) or the digits (48-57)
    c >= 33 && c <= 64 && c != 46 && (!is_digit(c))
}

fn is_digit(c: u8) -> bool {
    c >= 48 && c <= 57
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = get_grid(input);

    // get all special characters and their index
    let mut special_chars = Vec::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if is_special_char(c) {
                special_chars.push((i, j, c));
            }
        }
    }

    let mut sum = 0;
    // check every adjacent (also diagonal) byte of every special character if they are a digit
    // if so, go left and right of the digit to find the full number and add it to the sum
    // repeat for every special character and every adjacent digit
    for (i, j, _) in special_chars.iter() {
        let mut adjacent_digits: Vec<(usize, usize, u8)> = Vec::new();
        for i_offset in -1..=1 {
            let mut last_byte_was_digit = false;
            for j_offset in -1..=1 {
                if i_offset == 0 && j_offset == 0 {
                    last_byte_was_digit = false;
                    continue;
                }

                let i = *i as i32 + i_offset;
                let j = *j as i32 + j_offset;

                if i < 0 || j < 0 {
                    continue;
                }

                let i = i as usize;
                let j = j as usize;

                if i >= grid.len() || j >= grid[i].len() {
                    continue;
                }

                let c = grid[i][j];
                if is_digit(c) {
                    if last_byte_was_digit {
                        continue;
                    }
                    adjacent_digits.push((i, j, c));
                    last_byte_was_digit = true;
                } else {
                    last_byte_was_digit = false;
                }
            }
            last_byte_was_digit = false;
        }

        for (i, j, _) in adjacent_digits.iter() {
            let row = &grid[*i];
            let mut leftmost = *j;
            while leftmost > 0 && is_digit(row[leftmost - 1]) {
                leftmost -= 1;
            }

            let mut rightmost = *j;
            while rightmost < grid[*i].len() - 1 && is_digit(row[rightmost + 1]) {
                rightmost += 1;
            }

            let mut number = 0;
            for j in leftmost..=rightmost {
                let digit = row[j] - 48;
                number = number * 10 + digit as u32;
            }

            sum += number;
        }
    }

    Some(sum)
}

fn is_asterisk(c: u8) -> bool {
    c == 42
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = get_grid(input);

    // get all special characters and their index
    let mut special_chars = Vec::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if is_asterisk(c) {
                special_chars.push((i, j, c));
            }
        }
    }

    let mut sum = 0;
    // check every adjacent (also diagonal) byte of every special character if they are a digit
    // if so, go left and right of the digit to find the full number and add it to the sum
    // repeat for every special character and every adjacent digit
    for (i, j, _) in special_chars.iter() {
        let mut adjacent_digits: Vec<(usize, usize, u8)> = Vec::new();
        for i_offset in -1..=1 {
            let mut last_byte_was_digit = false;
            for j_offset in -1..=1 {
                if i_offset == 0 && j_offset == 0 {
                    last_byte_was_digit = false;
                    continue;
                }

                let i = *i as i32 + i_offset;
                let j = *j as i32 + j_offset;

                if i < 0 || j < 0 {
                    continue;
                }

                let i = i as usize;
                let j = j as usize;

                if i >= grid.len() || j >= grid[i].len() {
                    continue;
                }

                let c = grid[i][j];
                if is_digit(c) {
                    if last_byte_was_digit {
                        continue;
                    }
                    adjacent_digits.push((i, j, c));
                    last_byte_was_digit = true;
                } else {
                    last_byte_was_digit = false;
                }
            }
            last_byte_was_digit = false;
        }

        let mut numbers = vec![];
        for (i, j, _) in adjacent_digits.iter() {
            let row = &grid[*i];
            let mut leftmost = *j;
            while leftmost > 0 && is_digit(row[leftmost - 1]) {
                leftmost -= 1;
            }

            let mut rightmost = *j;
            while rightmost < grid[*i].len() - 1 && is_digit(row[rightmost + 1]) {
                rightmost += 1;
            }

            let mut number = 0;
            for j in leftmost..=rightmost {
                let digit = row[j] - 48;
                number = number * 10 + digit as u32;
            }

            numbers.push(number);
        }

        if numbers.len() == 2 {
            sum += numbers[0] * numbers[1];
        }
    }

    Some(sum)
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
