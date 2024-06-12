// leetcode.rs, My solutions to various Leetcode puzzles.
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

#[allow(unused_imports)]
use std::io::{self, BufRead, Read, Stdin};
use std::str::FromStr;

/// A trait for reading lines from a buffer, providing methods for various line operations.
pub trait LineRead {
    fn trimmed_lines(&mut self) -> impl Iterator<Item = String>;

    fn read_line_as<T: FromStr>(&mut self) -> Result<T, T::Err>;

    fn skip_line(&mut self);
}

impl<T: BufRead> LineRead for T {
    fn trimmed_lines(&mut self) -> impl Iterator<Item = String> {
        self.lines()
            .map(|l| l.expect("should read line from buffer").trim().to_string())
    }

    fn read_line_as<S: FromStr>(&mut self) -> Result<S, S::Err> {
        let mut buffer = String::new();
        self.read_line(&mut buffer)
            .expect("should read line from buffer");
        buffer.trim().to_string().parse::<S>()
    }

    fn skip_line(&mut self) {
        self.read_line(&mut "".to_string())
            .expect("should read line from buffer");
    }
}

#[derive(Debug)]
pub struct Vector<T: FromStr>(pub Vec<T>);

impl<T: FromStr> FromStr for Vector<T> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().trim_start_matches('[').trim_end_matches(']');
        if s.is_empty() {
            let v = vec![];
            return Ok(Vector(v));
        }
        let result: Result<Vec<T>, _> = s.split(',').map(|x| x.trim().parse::<T>()).collect();
        match result {
            Ok(vec) => Ok(Vector(vec)),
            Err(_) => Err("Failed to parse integers".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::io::Cursor;

    #[test]
    fn test_iter_lines() {
        let mut cursor = Cursor::new("  0  \n1 \n 2\n \n");
        let mut iter = cursor.trimmed_lines();
        assert_eq!(iter.next(), Some("0".to_string()));
        assert_eq!(iter.next(), Some("1".to_string()));
        assert_eq!(iter.next(), Some("2".to_string()));
        assert_eq!(iter.next(), Some("".to_string()));
    }

    #[test]
    fn test_read_line_as() {
        let mut cursor = Cursor::new("  0  \n1 \n 2\n");
        assert_eq!(cursor.read_line_as::<i32>(), Ok(0));
        assert_eq!(cursor.read_line_as::<i32>(), Ok(1));
        assert_eq!(cursor.read_line_as::<i32>(), Ok(2));

        let mut cursor = Cursor::new("  0  \n 3.1415 \n");
        assert_eq!(cursor.read_line_as::<f64>(), Ok(0.0));
        assert_eq!(cursor.read_line_as::<f64>(), Ok(3.1415));

        let mut cursor = Cursor::new("0\n1\n 2 \n");
        let i: Result<i32, _> = cursor.read_line_as();
        assert_eq!(i, Ok(0));

        let f: Result<f64, _> = cursor.read_line_as();
        assert_eq!(f, Ok(1.0_f64));

        let s: Result<String, _> = cursor.read_line_as();
        assert_eq!(s, Ok("2".to_string()));
    }

    #[test]
    #[should_panic]
    fn test_read_line_as_panic() {
        let mut cursor = Cursor::new("    \n  \n");
        assert_eq!(cursor.read_line_as::<f64>(), Ok(0.0));
    }

    #[test]
    fn test_skip_line() {
        let mut cursor = Cursor::new("\n\n");
        cursor.skip_line();
        cursor.skip_line();

        let mut cursor = Cursor::new("\n");
        cursor.skip_line();
        cursor.skip_line();
        cursor.skip_line();
        cursor.skip_line();
        cursor.skip_line();
    }

    #[test]
    fn test_vector_fromstr() -> Result<(), Box<dyn Error>> {
        let input = "[]";
        let Vector(v) = input.parse::<Vector<i32>>()?;
        assert_eq!(v, vec![]);

        let input = "  [ 1, 2,  3,  4,   5 ]  ";
        let Vector(v) = input.parse::<Vector<i32>>()?;
        assert_eq!(v, vec![1, 2, 3, 4, 5]);

        let input = "  [ -1, 2,  -3,  4,   5 ]  ";
        let Vector(v) = input.parse::<Vector<i32>>()?;
        assert_eq!(v, vec![-1, 2, -3, 4, 5]);

        let input = "  [ -1.0, 2,  -3,  4,   5.0 ]  ";
        let Vector(v) = input.parse::<Vector<f64>>()?;
        assert_eq!(v, vec![-1.0, 2.0, -3.0, 4.0, 5.0]);

        Ok(())
    }
}
