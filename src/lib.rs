use std::{iter::Enumerate, str::Lines};

pub enum CsvValue {
    Text(String),
    Integer(i64),
    Float(f64),
}

#[inline]
fn handle_new_key(key: &str, len: usize) -> String {
    let trimmed_key = key.trim();

    if trimmed_key.is_empty() {
        return format!("__{}", len + 1);
    }

    trimmed_key.to_string()
}

#[inline]
fn parse_header(lines: &mut Enumerate<Lines>, seperator: &char) -> Vec<String> {
    let mut keys = Vec::new();

    for (_, line) in lines {
        let trimmed_line = line.trim();

        if !trimmed_line.is_empty() {
            let mut current_key = String::new();

            for ch in trimmed_line.chars() {
                if &ch == seperator {
                    keys.push(handle_new_key(&current_key, keys.len()));

                    current_key.clear();
                } else {
                    current_key.push(ch);
                }
            }

            keys.push(handle_new_key(&current_key, keys.len()));

            break;
        }
    }

    keys
}

#[cfg(test)]
mod test_parse_header {
    use crate::parse_header;

    #[test]
    fn it_should_accept_any_seperator() {
        let keys = ["key 1", "key 2", "key 3", "key 4"];

        let seperators = [',', '\t', ';'];

        for seperator in seperators {
            let input = keys.join(&seperator.to_string());

            let result = parse_header(&mut input.lines().enumerate(), &seperator);

            for i in 0..keys.len() {
                assert_eq!(keys[i], result[i]);
            }
        }
    }

    #[test]
    fn it_should_trim_spaces() {
        let keys = ["    key 1    ", " key 2 ", "  key 3        ", "   key 4   "];

        let seperators = [',', '\t', ';'];

        for seperator in seperators {
            let input = keys.join(&seperator.to_string());

            let result = parse_header(&mut input.lines().enumerate(), &seperator);

            for i in 0..keys.len() {
                assert_eq!(keys[i].trim(), result[i]);
            }
        }
    }

    #[test]
    fn it_should_ignore_empty_lines() {
        let keys = ["key 1", "key 2", "key 3", "key 4"];

        let seperators = [',', '\t', ';'];

        for seperator in seperators {
            let input = format!("\n\n\n{}", keys.join(&seperator.to_string()));

            let result = parse_header(&mut input.lines().enumerate(), &seperator);

            for i in 0..keys.len() {
                assert_eq!(keys[i].trim(), result[i]);
            }
        }
    }

    #[test]
    fn it_should_not_touch_lines_after() {
        let keys = ["key 1", "key 2", "key 3", "key 4"];

        let seperators = [',', '\t', ';'];

        for seperator in seperators {
            let mut input = keys.join(&seperator.to_string());

            let mut extra_lines = Vec::new();

            for i in 0..keys.len() {
                let mut current_line = String::new();

                for j in 0..keys.len() {
                    if j > 0 {
                        current_line.push(seperator);
                    }

                    current_line.push_str(&format!("value {i} {j}"));
                }

                input.push_str(&format!("\n{current_line}"));
                extra_lines.push(current_line);
            }

            let mut lines = input.lines().enumerate();

            let result = parse_header(&mut lines, &seperator);

            for i in 0..keys.len() {
                assert_eq!(keys[i], result[i]);
            }

            for extra_line in extra_lines {
                let (_, l) = lines.next().expect("it to not be touched");

                assert_eq!(extra_line, l);
            }
        }
    }
}

pub fn parse(input: &str) -> Vec<std::collections::HashMap<String, Option<CsvValue>>> {
    let mut output = Vec::new();

    let seperator = ',';

    let mut lines = input.lines().enumerate();

    let header = parse_header(&mut lines, &seperator);

    output
}

#[cfg(test)]
mod test_parser {}
