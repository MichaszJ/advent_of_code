use std::fs;

const NUMBERS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn find_calibration_value(line: &str, parse_words: bool) -> usize {
    let mut number_idxs: Vec<(usize, usize)> = Vec::new();

    if parse_words {
        for (num_val, num) in NUMBERS.iter().enumerate() {
            let matches: Vec<(usize, &str)> = line.match_indices(num).collect();
            if !matches.is_empty() {
                for (index, _) in matches {
                    number_idxs.push((num_val, index));
                }
            }
        }
    }

    for (index, char) in line.chars().enumerate() {
        if char.is_ascii_digit() {
            number_idxs.push((char.to_digit(10).unwrap() as usize, index));
        }
    }

    number_idxs.sort_by(|a, b| a.1.cmp(&b.1));

    let first_num = number_idxs.first().unwrap().0;
    let last_num = number_idxs.last().unwrap().0;

    format!("{first_num}{last_num}").parse::<usize>().unwrap()
}

fn main() {
    let data = fs::read_to_string("data.txt").unwrap();
    let lines: Vec<&str> = data.split('\n').collect();

    let part1_answer: usize = lines
        .clone()
        .into_iter()
        .map(|line| find_calibration_value(line, false))
        .sum();

    println!("Part 1 Answer: {part1_answer}");

    let part2_answer: usize = lines
        .clone()
        .into_iter()
        .map(|line| find_calibration_value(line, true))
        .sum();

    println!("Part 2 Answer: {part2_answer}");
}

#[cfg(test)]
mod tests {
    use crate::find_calibration_value;

    #[test]
    fn only_digits() {
        let result11 = find_calibration_value("1abc2", false);
        assert_eq!(result11, 12);

        let result12 = find_calibration_value("pqr3stu8vwx", false);
        assert_eq!(result12, 38);

        let result13 = find_calibration_value("a1b2c3d4e5f", false);
        assert_eq!(result13, 15);

        let result14 = find_calibration_value("treb7uchet", false);
        assert_eq!(result14, 77);
    }

    #[test]
    fn digits_and_words() {
        let result21 = find_calibration_value("two1nine", true);
        assert_eq!(result21, 29);

        let result22 = find_calibration_value("eightwothree", true);
        assert_eq!(result22, 83);

        let result23 = find_calibration_value("abcone2threexyz", true);
        assert_eq!(result23, 13);

        let result24 = find_calibration_value("xtwone3four", true);
        assert_eq!(result24, 24);

        let result25 = find_calibration_value("4nineeightseven2", true);
        assert_eq!(result25, 42);

        let result26 = find_calibration_value("zoneight234", true);
        assert_eq!(result26, 14);

        let result27 = find_calibration_value("7pqrstsixteen", true);
        assert_eq!(result27, 76);
    }
}
