use std::{collections::HashMap, iter::Enumerate, str::Lines};

#[derive(Debug)]
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
fn parse_header(lines: &mut Enumerate<Lines>, seperator: &char) -> Option<Vec<String>> {
    for (_, line) in lines {
        let mut keys = Vec::new();

        let mut current_key = String::new();

        for ch in line.chars() {
            if &ch == seperator {
                keys.push(handle_new_key(&current_key, keys.len()));

                current_key.clear();
            } else {
                current_key.push(ch);
            }
        }

        if !keys.is_empty() {
            keys.push(handle_new_key(&current_key, keys.len()));

            return Some(keys);
        }
    }

    None
}

#[cfg(test)]
mod test_parse_header {
    use crate::parse_header;

    const SEPERATORS: [char; 3] = ['\t', ',', ';'];

    #[test]
    fn it_should_accept_any_seperator() {
        let keys = ["key 1", "key 2", "key 3", "key 4"];

        for seperator in SEPERATORS {
            let input = keys.join(&seperator.to_string());

            let result = parse_header(&mut input.lines().enumerate(), &seperator)
                .expect("it to return a value");

            for i in 0..keys.len() {
                assert_eq!(keys[i], result[i]);
            }
        }
    }

    #[test]
    fn it_should_trim_spaces() {
        let keys = ["    key 1    ", " key 2 ", "  key 3        ", "   key 4   "];

        for seperator in SEPERATORS {
            let input = keys.join(&seperator.to_string());

            let result = parse_header(&mut input.lines().enumerate(), &seperator)
                .expect("it to return a value");

            for i in 0..keys.len() {
                assert_eq!(keys[i].trim(), result[i]);
            }
        }
    }

    #[test]
    fn it_should_ignore_empty_lines() {
        let keys = ["key 1", "key 2", "key 3", "key 4"];

        for seperator in SEPERATORS {
            let input = format!("\n\n\n{}", keys.join(&seperator.to_string()));

            let result = parse_header(&mut input.lines().enumerate(), &seperator)
                .expect("it to return a value");

            for i in 0..keys.len() {
                assert_eq!(keys[i].trim(), result[i]);
            }
        }
    }

    #[test]
    fn it_should_not_touch_lines_after() {
        let keys = ["key 1", "key 2", "key 3", "key 4"];

        for seperator in SEPERATORS {
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

            let result = parse_header(&mut lines, &seperator).expect("it to return a value");

            for i in 0..keys.len() {
                assert_eq!(keys[i], result[i])
            }

            for extra_line in extra_lines {
                let (_, l) = lines.next().expect("it to not be touched");

                assert_eq!(extra_line, l);
            }
        }
    }

    #[test]
    fn it_should_generate_unknown_fields() {
        let fields = ["", "", "", "", ""];

        for seperator in SEPERATORS {
            let sep_str = seperator.to_string();
            println!("sep_str: '{sep_str}'");

            let input = fields.join(&sep_str);

            let result = parse_header(&mut input.lines().enumerate(), &seperator)
                .expect("it to return a value");

            for i in 0..fields.len() {
                assert_eq!(format!("__{}", i + 1), result[i]);
            }
        }
    }
}

#[inline]
fn parse_value(value: &str) -> Option<CsvValue> {
    let trimmed_line = value.trim();

    if trimmed_line.is_empty() {
        return None;
    }

    if let Ok(integer) = trimmed_line.parse::<i64>() {
        return Some(CsvValue::Integer(integer));
    }

    if let Ok(maybe_float) = trimmed_line.parse::<f64>() {
        return Some(CsvValue::Float(maybe_float));
    }

    Some(CsvValue::Text(trimmed_line.to_string()))
}

#[cfg(test)]
mod test_parse_value {
    use crate::{parse_value, CsvValue};

    #[test]
    fn it_should_understand_missing_values() {
        let values = ["", "    "];

        for value in values {
            assert!(parse_value(value).is_none());
        }
    }

    #[test]
    fn it_should_understand_strings_values() {
        let values = [
            "   value 1   ",
            "value 2 ",
            "value 3 ",
            "   value 4 ",
            "value 5",
        ];

        for value in values {
            let result = parse_value(&value.to_string());

            match result {
                Some(CsvValue::Text(result_value)) => assert_eq!(value.trim(), result_value),
                invalid_result => panic!(
                    "expected it to return CsvValue::Text({}) but received '{invalid_result:?}'",
                    value.trim()
                ),
            }
        }
    }

    #[test]
    fn it_should_understand_integers_values() {
        let values = [-1, 0, 1, 2, 3, 4, 5];

        for value in values {
            let result = parse_value(&value.to_string());

            match result {
                Some(CsvValue::Integer(result_value)) => assert_eq!(value, result_value),
                invalid_result => panic!(
                    "expected it to return CsvValue::Integer({value}) but received '{invalid_result:?}'"
                ),
            }
        }
    }

    #[test]
    fn it_should_understand_floats_values() {
        let values = [-1.1, 1.1, 2.22, 3.33, 4.44, 5.55];

        for value in values {
            match parse_value(&value.to_string()) {
                Some(CsvValue::Float(result_value)) => assert_eq!(value, result_value),
                invalid_result => {
                    panic!(
                        "expected it to return CsvValue::Float({value}) but received '{invalid_result:?}'"
                    )
                }
            }
        }
    }

    #[test]
    fn zero_point_zero_should_be_a_float() {
        match parse_value("0.0") {
            Some(CsvValue::Float(value)) => assert_eq!(0.0, value),
            invalid_result => panic!(
                "expected it to return CsvValue::Float(0.0) but received '{invalid_result:?}'"
            ),
        };
    }
}

#[inline]
fn get_value_field(fields: &[String], index: usize) -> String {
    fields
        .get(index)
        .map(|f| f.to_string())
        .unwrap_or_else(|| format!("__{}", index + 1))
}

#[cfg(test)]
mod test_get_value_field {
    use crate::get_value_field;

    #[test]
    fn it_should_generate_missing_fields() {
        let fields = vec![
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
            "4".to_string(),
            "5".to_string(),
        ];

        for i in 0..fields.len() * 2 {
            assert_eq!(
                get_value_field(&fields, i),
                if i < fields.len() {
                    fields[i].clone()
                } else {
                    format!("__{}", i + 1)
                }
            )
        }
    }
}

#[inline]
fn parse_value_line(line: &str, seperator: char, fields: &[String]) -> HashMap<String, CsvValue> {
    let mut values = std::collections::HashMap::new();

    let mut current_value = String::new();

    let mut index = 0;

    for ch in line.chars() {
        if ch == seperator {
            if let Some(value) = parse_value(&current_value) {
                values.insert(get_value_field(fields, index), value);
            }

            current_value.clear();
            index += 1;
        } else {
            current_value.push(ch);
        }
    }

    if let Some(value) = parse_value(&current_value) {
        values.insert(get_value_field(fields, index), value);
    }

    values
}

#[cfg(test)]
mod test_parse_value_line {
    use crate::{parse_value_line, CsvValue};

    const SEPERATORS: [char; 3] = ['\t', ',', ';'];

    #[test]
    fn it_should_parse_the_line() {
        let fields = vec![
            "key 1".to_string(),
            "key 2".to_string(),
            "key 3".to_string(),
            "key 4".to_string(),
            "key 5".to_string(),
        ];

        let values = ["text", "", "1", "1.1", ""];

        for sep in SEPERATORS {
            let line = values.join(&sep.to_string());

            let result = parse_value_line(&line, sep, &fields);
            println!("result:{result:?}");

            match result.get(&fields[0]) {
                Some(CsvValue::Text(result_value)) => assert_eq!(result_value, values[0]),
                _ => panic!(),
            };

            match result.get(&fields[1]) {
                None => {}
                _ => panic!(),
            };

            match result.get(&fields[2]) {
                Some(CsvValue::Integer(result_value)) => assert_eq!(*result_value, 1),
                _ => panic!(),
            };

            match result.get(&fields[3]) {
                Some(CsvValue::Float(result_value)) => assert_eq!(*result_value, 1.1),
                _ => panic!(),
            };

            match result.get(&fields[4]) {
                None => {}
                _ => panic!(),
            };
        }
    }

    #[test]
    fn it_should_generate_unknown_field_names() {
        let fields = Vec::new();

        let values = ["text", "", "1", "1.1", ""];

        for sep in SEPERATORS {
            let line = values.join(&sep.to_string());

            let result = parse_value_line(&line, sep, &fields);
            println!("result:{result:?}");

            match result.get("__1") {
                Some(CsvValue::Text(result_value)) => assert_eq!(result_value, values[0]),
                _ => panic!(),
            };

            match result.get("__2") {
                None => {}
                _ => panic!(),
            };

            match result.get("__3") {
                Some(CsvValue::Integer(result_value)) => assert_eq!(*result_value, 1),
                _ => panic!(),
            };

            match result.get("__4") {
                Some(CsvValue::Float(result_value)) => assert_eq!(*result_value, 1.1),
                _ => panic!(),
            };

            match result.get("__5") {
                None => {}
                _ => panic!(),
            };
        }
    }
}

pub fn parse_csv(input: &str, seperator: char) -> Vec<HashMap<String, CsvValue>> {
    let mut output = Vec::new();

    let mut lines = input.lines().enumerate();

    if let Some(fields) = parse_header(&mut lines, &seperator) {
        for (_, line) in lines {
            let trimmed_line = line.trim();

            if !trimmed_line.is_empty() {
                output.push(parse_value_line(trimmed_line, seperator, &fields))
            }
        }
    }

    output
}

#[cfg(test)]
mod test_parse_csv {
    use crate::{parse_csv, CsvValue};

    const SEPERATORS: [char; 3] = ['\t', ',', ';'];

    #[test]
    fn it_should_be_able_to_parse_csv_files_with_integers() {
        let fields = ["text", "integer", "float", "missng"];

        let text_values = ["   mads", "was    ", "    here   "];
        let integer_values = [-1, 0, 2];
        let float_values = [-1.1, 1.1, 2.2];

        for seperator in SEPERATORS {
            let sep_str = seperator.to_string();

            let mut input = fields.join(&sep_str);

            for i in 0..text_values.len() {
                input.push_str(&format!(
                    "\n{}{seperator}{}{seperator}{}{seperator}",
                    text_values[i], integer_values[i], float_values[i]
                ));
            }

            let result = parse_csv(&input, seperator);

            assert_eq!(result.len(), text_values.len());

            for i in 0..text_values.len() {
                let col = &result[i];

                match col.get(fields[0]) {
                    Some(CsvValue::Text(value)) => assert_eq!(value, text_values[i].trim()),
                    invalid_value => panic!(
                        "Expected to CsvValue::Text({}), but received {invalid_value:?}",
                        text_values[i].trim()
                    ),
                };

                match col.get(fields[1]) {
                    Some(CsvValue::Integer(value)) => assert_eq!(*value, integer_values[i]),
                    invalid_value => panic!(
                        "Expected to CsvValue::Integer({}), but received {invalid_value:?}",
                        integer_values[i]
                    ),
                };

                match col.get(fields[2]) {
                    Some(CsvValue::Float(value)) => assert_eq!(*value, float_values[i]),
                    invalid_value => panic!(
                        "Expected to CsvValue::Float({}), but received {invalid_value:?}",
                        float_values[i]
                    ),
                };

                match col.get(fields[3]) {
                    None => {}
                    invalid_value => panic!("Expected to None, but received {invalid_value:?}"),
                };
            }
        }
    }
}
