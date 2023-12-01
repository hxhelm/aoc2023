advent_of_code::solution!(1);

fn get_digits(input: &str) -> Vec<u32> {
    input
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let digits_lines = input
        .lines()
        .map(|line| get_digits(line))
        .collect::<Vec<Vec<u32>>>();

    let mut sum = 0;
    for line in digits_lines.iter() {
        if line.len() == 0 {
            continue;
        } else if line.len() == 1 {
            sum += (line[0] * 10) + line[0];
        } else {
            sum += (line.first().unwrap() * 10) + line.last().unwrap();
        }
    }

    Some(sum)
}

fn get_digits_from_words_or_numbers(input: &str) -> Vec<u32> {
    let mut digits = Vec::new();

    let mut word_buffer = String::new();

    for c in input.chars() {
        if c.is_digit(10) {
            word_buffer = String::new();
            digits.push(c.to_digit(10).unwrap());
            continue;
        }

        word_buffer.push(c);

        let digit_from = match word_buffer.as_str() {
            word if word.contains("one") => Some(1),
            word if word.contains("two") => Some(2),
            word if word.contains("three") => Some(3),
            word if word.contains("four") => Some(4),
            word if word.contains("five") => Some(5),
            word if word.contains("six") => Some(6),
            word if word.contains("seven") => Some(7),
            word if word.contains("eight") => Some(8),
            word if word.contains("nine") => Some(9),
            _ => None,
        };

        if digit_from.is_some() {
            word_buffer = word_buffer.pop().unwrap().to_string();
            digits.push(digit_from.unwrap());
        }
    }

    println!("{:?}", &digits);

    digits
}

pub fn part_two(input: &str) -> Option<u32> {
    let digits_lines = input
        .lines()
        .map(|line| get_digits_from_words_or_numbers(line))
        .collect::<Vec<Vec<u32>>>();

    let mut sum = 0;
    for line in digits_lines.iter() {
        if line.len() == 0 {
            continue;
        } else if line.len() == 1 {
            sum += (line[0] * 10) + line[0];
        } else {
            sum += (line.first().unwrap() * 10) + line.last().unwrap();
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let example_1 = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let result = part_one(example_1);
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
