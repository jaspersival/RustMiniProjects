/*
Write a program which performs addition, subtraction, multiplication of matrices. The dimensions of
 both the matrices would be specified by the user (dynamic memory allocation required).
 Use of structure or a class to define the matrix would be a good idea. (Intermediate)
 */
use std::fmt;
use std::io;
use std::str::FromStr;

fn main() -> io::Result<()> {
    let mut user_input = String::new();
    println!("Add, subtract or multiply matrices:");
    let stdin = io::stdin();
    stdin.read_line(&mut user_input).unwrap();
    println!("input {}", user_input);

    for character in user_input.chars() {
        println!("Character: {}", character);
    }
    Ok(())
}

fn parse_matrix<T>(s: &str) -> Vec<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: fmt::Debug,
{
    let trimmed_string = s.trim();
    let matrix_string_without_brackets = trimmed_string.get(1..trimmed_string.len() - 1).unwrap();
    matrix_string_without_brackets
        .split(';')
        .map(|row| {
            row.split(',')
                .map(|element| element.trim().parse().unwrap())
                .collect()
        })
        .collect()
}

#[test]
fn parse_string_with_whitespace_between_rows_into_matrix_correctly() {
    let user_input = "[1,2,3; 4,5,6]";

    let expected = vec![vec![1, 2, 3], vec![4, 5, 6]];

    let result: Vec<Vec<i64>> = parse_matrix(user_input);

    assert_eq!(result, expected);
}
#[test]
fn parse_string_without_whitespace_between_rows_into_matrix_correctly() {
    let user_input = "[1,2,3;4,5,6]";

    let expected = vec![vec![1, 2, 3], vec![4, 5, 6]];

    let result: Vec<Vec<i64>> = parse_matrix(user_input);

    assert_eq!(result, expected);
}
#[test]
fn parse_string_with_leading_space_correctly() {
    let user_input = " [1,2,3;4,5,6]";

    let expected = vec![vec![1, 2, 3], vec![4, 5, 6]];

    let result: Vec<Vec<i64>> = parse_matrix(user_input);

    assert_eq!(result, expected);
}
#[test]
fn parse_string_with_trailing_space_correctly() {
    let user_input = "[1,2,3;4,5,6] ";

    let expected = vec![vec![1, 2, 3], vec![4, 5, 6]];

    let result: Vec<Vec<i64>> = parse_matrix(user_input);

    assert_eq!(result, expected);
}
#[test]
fn parse_string_with_floats_correctly() {
    let user_input = "[1.0,2.5,3.2;4,5,6] ";

    let expected = vec![vec![1.0, 2.5, 3.2], vec![4.0, 5.0, 6.0]];

    let result: Vec<Vec<f64>> = parse_matrix(user_input);

    assert_eq!(result, expected);
}
