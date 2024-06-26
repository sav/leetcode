// leetcode.rs, My solutions to various Leetcode puzzles.
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

#[allow(unused_imports)]
use std::io::{self, BufRead, Read, Stdin};
use std::str::FromStr;

/// A trait for reading lines from a buffer, providing methods for various line operations.
pub trait InputReader {
    fn trimmed_lines(&mut self) -> impl Iterator<Item = String>;

    fn skip_line(&mut self);

    fn read_as<T: FromStr>(&mut self) -> Result<T, T::Err>;

    fn read_vec_of<T: FromStr>(&mut self) -> Result<Vec<T>, T::Err>;

    fn read_matrix_of<T: FromStr>(&mut self) -> Result<Vec<Vec<T>>, T::Err>;
}

impl<B: BufRead> InputReader for B {
    fn trimmed_lines(&mut self) -> impl Iterator<Item = String> {
        self.lines()
            .map(|l| l.expect("should read line from buffer").trim().to_string())
    }

    fn skip_line(&mut self) {
        self.read_line(&mut "".to_string())
            .expect("should read line from buffer");
    }

    fn read_as<T: FromStr>(&mut self) -> Result<T, T::Err> {
        let mut buffer = String::new();
        self.read_line(&mut buffer)
            .expect("should read line from buffer");
        buffer.trim().to_string().parse::<T>()
    }

    fn read_vec_of<T: FromStr>(&mut self) -> Result<Vec<T>, T::Err> {
        let mut buffer = String::new();
        self.read_line(&mut buffer)
            .expect("should read line from buffer");
        buffer
            .trim_matches(|c| c == ' ' || c == '\t' || c == '[' || c == ']')
            .split(',')
            .map(|s| s.trim_matches(|c| c == '"' || c == ' ' || c == '\''))
            .map(|s| s.parse::<T>())
            .collect::<Result<Vec<T>, T::Err>>()
    }

    fn read_matrix_of<T: FromStr>(&mut self) -> Result<Vec<Vec<T>>, T::Err> {
        let mut result = Vec::new();
        for line in self.lines() {
            if let Ok(line) = line {
                let elements = line
                    .trim_matches(|c| c == '[' || c == ']' || c == ',' || c == ' ' || c == '\t')
                    .split(',')
                    .map(|s| s.trim_matches(|c| c == '"' || c == ' ' || c == '\''))
                    .map(|s| s.parse::<T>())
                    .collect::<Result<Vec<T>, T::Err>>()?;
                result.push(elements);
            }
        }
        Ok(result)
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

#[derive(Debug)]
pub struct Matrix<T: std::fmt::Display>(pub Vec<Vec<T>>);

impl<T: std::fmt::Display> std::fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            for elem in row.iter() {
                write!(f, "{:>4}", elem)?;
            }
            writeln!(f)?;
        }
        Ok(())
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
        assert_eq!(cursor.read_as::<i32>(), Ok(0));
        assert_eq!(cursor.read_as::<i32>(), Ok(1));
        assert_eq!(cursor.read_as::<i32>(), Ok(2));

        let mut cursor = Cursor::new("  0  \n 3.1415 \n");
        assert_eq!(cursor.read_as::<f64>(), Ok(0.0));
        assert_eq!(cursor.read_as::<f64>(), Ok(3.1415));

        let mut cursor = Cursor::new("0\n1\n 2 \n");
        let i: Result<i32, _> = cursor.read_as();
        assert_eq!(i, Ok(0));

        let f: Result<f64, _> = cursor.read_as();
        assert_eq!(f, Ok(1.0_f64));

        let s: Result<String, _> = cursor.read_as();
        assert_eq!(s, Ok("2".to_string()));
    }

    #[test]
    #[should_panic]
    fn test_read_line_as_panic() {
        let mut cursor = Cursor::new("    \n  \n");
        assert_eq!(cursor.read_as::<f64>(), Ok(0.0));
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

    #[test]
    fn test_read_vec_as() {
        let mut cursor = Cursor::new("[\"5\",\"3\"]");
        let mat1 = cursor.read_vec_of::<i32>();
        assert_eq!(mat1, Ok(vec![5, 3]));

        let mut cursor = Cursor::new("  [ '3' , '5' ] ");
        let mat1 = cursor.read_vec_of::<i32>();
        assert_eq!(mat1, Ok(vec![3, 5]));
    }

    #[test]
    fn test_read_matrix_as() {
        let mut cursor = Cursor::new("[[\"5\",\"3\"]\n,[\"6\",\".\"]\n,[\".\",\"9\"]]\n");
        let mat1 = cursor.read_matrix_of::<String>();
        assert_eq!(
            mat1,
            Ok(vec![
                vec!["5".into(), "3".into()],
                vec!["6".into(), ".".into()],
                vec![".".into(), "9".into()]
            ])
        );

        let mut cursor = Cursor::new("[[\"5\",\"3\"],\n [\"6\",\".\"],\n [\".\",\"9\"]]");
        let mat1 = cursor.read_matrix_of::<String>();
        assert_eq!(
            mat1,
            Ok(vec![
                vec!["5".into(), "3".into()],
                vec!["6".into(), ".".into()],
                vec![".".into(), "9".into()]
            ])
        );

        let mut cursor = Cursor::new("\t[[\"5\",\"3\"],\n\t\t[\"6\",\".\"],\n\t\t[\".\",\"9\"]]");
        let mat1 = cursor.read_matrix_of::<String>();
        assert_eq!(
            mat1,
            Ok(vec![
                vec!["5".into(), "3".into()],
                vec!["6".into(), ".".into()],
                vec![".".into(), "9".into()]
            ])
        );
    }
}
