use core::{iter::Enumerate, str::Lines};

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

    trimmed_key.to_owned()
}

#[inline]
fn parse_header(lines: &mut Enumerate<Lines>, separator: char) -> Option<Vec<String>> {
    for (_, line) in lines {
        let mut keys = Vec::new();

        let mut current_key = String::new();

        for ch in line.chars() {
            if ch == separator {
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

    const SEPARATORS: [char; 3] = ['\t', ',', ';'];

    #[test]
    fn it_should_accept_any_separator() {
        let keys = ["key 1", "key 2", "key 3", "key 4"];

        for separator in SEPARATORS {
            let input = keys.join(&separator.to_string());

            let result = parse_header(&mut input.lines().enumerate(), separator)
                .expect("it to return a value");

            for i in 0..keys.len() {
                let key = keys.get(i).expect("it to exist").trim();
                let value = result.get(i).expect("it to exist");
                assert_eq!(key, value);
            }
        }
    }

    #[test]
    fn it_should_trim_spaces() {
        let keys = ["    key 1    ", " key 2 ", "  key 3        ", "   key 4   "];

        for separator in SEPARATORS {
            let input = keys.join(&separator.to_string());

            let result = parse_header(&mut input.lines().enumerate(), separator)
                .expect("it to return a value");

            for i in 0..keys.len() {
                let key = keys.get(i).expect("it to exist").trim();
                let value = result.get(i).expect("it to exist");
                assert_eq!(key, value);
            }
        }
    }

    #[test]
    fn it_should_ignore_empty_lines() {
        let keys = ["key 1", "key 2", "key 3", "key 4"];

        for separator in SEPARATORS {
            let input = format!("\n\n\n{}", keys.join(&separator.to_string()));

            let result = parse_header(&mut input.lines().enumerate(), separator)
                .expect("it to return a value");

            for i in 0..keys.len() {
                let key = keys.get(i).expect("it to exist").trim();
                let value = result.get(i).expect("it to exist");

                assert_eq!(key, value);
            }
        }
    }

    #[test]
    fn it_should_return_none_if_empty_line() {
        for separator in SEPARATORS {
            let input = String::new();
            let mut lines = input.lines().enumerate();

            assert!(parse_header(&mut lines, separator).is_none());
        }
    }

    #[test]
    fn it_should_not_touch_lines_after() {
        let keys = ["key 1", "key 2", "key 3", "key 4"];

        for separator in SEPARATORS {
            let mut input = keys.join(&separator.to_string());

            let mut extra_lines = Vec::new();

            for i in 0..keys.len() {
                let mut current_line = String::new();

                for j in 0..keys.len() {
                    if j > 0 {
                        current_line.push(separator);
                    }

                    current_line.push_str(&format!("value {i} {j}"));
                }

                input.push_str(&format!("\n{current_line}"));
                extra_lines.push(current_line);
            }

            let mut lines = input.lines().enumerate();

            let result = parse_header(&mut lines, separator).expect("it to return a value");

            for i in 0..keys.len() {
                assert_eq!(
                    keys.get(i).expect("it to exist"),
                    result.get(i).expect("it to exist")
                );
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

        for separator in SEPARATORS {
            let sep_str = separator.to_string();

            let input = fields.join(&sep_str);

            let result = parse_header(&mut input.lines().enumerate(), separator)
                .expect("it to return a value");

            for i in 0..fields.len() {
                assert_eq!(&format!("__{}", i + 1), result.get(i).expect("it to exist"));
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

    Some(CsvValue::Text(trimmed_line.to_owned()))
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
            let result = parse_value(value).expect("it to be some");

            assert!(matches!(result, CsvValue::Text(v) if v == value.trim()));
        }
    }

    #[test]
    fn it_should_understand_integers_values() {
        let values: [i64; 7] = [-1, 0, 1, 2, 3, 4, 5];

        for value in values {
            let result = parse_value(&value.to_string()).expect("it to be some");

            assert!(matches!(result, CsvValue::Integer(v) if v == value));
        }
    }

    #[test]
    fn it_should_understand_floats_values() {
        let values: [f64; 6] = [-1.1, 1.1, 2.22, 3.33, 4.44, 5.55];

        for value in values {
            let result = parse_value(&value.to_string()).expect("it to be some");

            assert!(matches!(result, CsvValue::Float(v) if (v - value).abs() < f64::EPSILON));
        }
    }

    #[test]
    fn zero_point_zero_should_be_a_float() {
        let result = parse_value("0.0").expect("it to be some");

        assert!(matches!(result, CsvValue::Float(value) if value == 0.0f64));
    }
}

#[inline]
fn get_value_field(fields: &[String], index: usize) -> String {
    fields.get(index).map_or_else(
        || format!("__{}", index + 1),
        std::string::ToString::to_string,
    )
}

#[cfg(test)]
mod test_get_value_field {
    use crate::get_value_field;

    #[test]
    fn it_should_generate_missing_fields() {
        let fields = vec![
            "1".to_owned(),
            "2".to_owned(),
            "3".to_owned(),
            "4".to_owned(),
            "5".to_owned(),
        ];

        for i in 0..fields.len() * 2 {
            let result = get_value_field(&fields, i);
            if i < fields.len() {
                assert_eq!(&result, fields.get(i).expect("it to be some"));
            } else {
                assert_eq!(result, format!("__{}", i + 1));
            }
        }
    }
}

#[inline]
fn parse_value_line(
    line: &str,
    separator: char,
    fields: &[String],
) -> std::collections::HashMap<String, CsvValue> {
    let mut values = std::collections::HashMap::new();

    let mut current_value = String::new();

    let mut index = 0;

    for ch in line.chars() {
        if ch == separator {
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

    const SEPARATORS: [char; 3] = ['\t', ',', ';'];

    #[test]
    fn it_should_parse_the_line() {
        let fields = vec![
            "key 1".to_owned(),
            "key 2".to_owned(),
            "key 3".to_owned(),
            "key 4".to_owned(),
            "key 5".to_owned(),
        ];

        let values = ["text", "", "1", "1.1", ""];

        for sep in SEPARATORS {
            let line = values.join(&sep.to_string());

            let result = parse_value_line(&line, sep, &fields);

            let one = result
                .get(fields.first().expect("it to be some"))
                .expect("it to be some");
            assert!(matches!(one, CsvValue::Text(result_value) if result_value == values[0]));

            assert!(result.get(fields.get(1).expect("it to be some")).is_none());

            let three = result
                .get(fields.get(2).expect("it to be some"))
                .expect("it to be some");
            assert!(matches!(three, CsvValue::Integer(result_value) if *result_value == 1i64));

            let four = result
                .get(fields.get(3).expect("it to be some"))
                .expect("it to be some");
            assert!(
                matches!(four, CsvValue::Float(result_value) if (*result_value - 1.1f64).abs() < f64::EPSILON)
            );

            assert!(result.get(fields.get(4).expect("it to be some")).is_none());
        }
    }

    #[test]
    fn it_should_generate_unknown_field_names() {
        let fields = Vec::new();

        let values = ["text", "", "1", "1.1", ""];

        for sep in SEPARATORS {
            let line = values.join(&sep.to_string());

            let result = parse_value_line(&line, sep, &fields);

            let one = result.get("__1").expect("it to be some");
            assert!(matches!(one, CsvValue::Text(result_value) if result_value == values[0]));

            assert!(result.get("__2").is_none());

            let three = result.get("__3").expect("it to be some");
            assert!(matches!(three, CsvValue::Integer(result_value) if *result_value == 1i64));

            let four = result.get("__4").expect("it to be some");
            assert!(
                matches!(four, CsvValue::Float(result_value) if (*result_value - 1.1f64).abs() < f64::EPSILON)
            );

            assert!(result.get("__5").is_none());
        }
    }
}

#[inline]
pub fn parse_csv(input: &str, separator: char) -> Vec<std::collections::HashMap<String, CsvValue>> {
    let mut output = Vec::new();

    let mut lines = input.lines().enumerate();

    if let Some(fields) = parse_header(&mut lines, separator) {
        for (_, line) in lines {
            let trimmed_line = line.trim();

            if !trimmed_line.is_empty() {
                output.push(parse_value_line(trimmed_line, separator, &fields));
            }
        }
    }

    output
}

#[cfg(test)]
mod test_parse_csv {
    use crate::{parse_csv, CsvValue};

    const SEPARATORS: [char; 3] = ['\t', ',', ';'];

    #[test]
    fn it_should_be_able_to_parse_csv_files_with_integers() {
        let fields = ["text", "integer", "float", "missng"];

        let text_values = ["   mads", "was    ", "    here   "];
        let integer_values: [i64; 3] = [-1, 0, 2];
        let float_values: [f64; 3] = [-1.1, 1.1, 2.2];

        for separator in SEPARATORS {
            let sep_str = separator.to_string();

            let mut input = fields.join(&sep_str);

            for i in 0..text_values.len() {
                input.push_str(&format!(
                    "\n{}{separator}{}{separator}{}{separator}",
                    text_values.get(i).expect("it to be some"),
                    integer_values.get(i).expect("it to be some"),
                    float_values.get(i).expect("it to be some")
                ));
            }

            let result = parse_csv(&input, separator);

            assert_eq!(result.len(), text_values.len());

            for i in 0..text_values.len() {
                let col = &result.get(i).expect("it to be some");

                let zero = col.get(fields[0]).expect("it to be some");
                let expected_string = text_values.get(i).expect("it to be some").trim();
                assert!(matches!(zero, CsvValue::Text(value) if value == expected_string));

                let one = col.get(fields[1]).expect("it to be some");
                let expected_integer = integer_values.get(i).expect("it to be some");
                assert!(matches!(one, CsvValue::Integer(value) if value == expected_integer));

                let two = col.get(fields[2]).expect("it to be some");
                let expected_float = float_values.get(i).expect("it to be some");
                assert!(
                    matches!(two, CsvValue::Float(value) if (value - expected_float).abs() < f64::EPSILON)
                );

                assert!(col.get(fields[3]).is_none());
            }
        }
    }
}
