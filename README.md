# csvvy

csvvy is a very simple csv parser that you most likely shouldn't use.

If you for some _weird_ reason want to use it; it should be pretty straightforward:

```rust
fn do_something() {
    let input = "
name, height, weight
Mads, 174, 62.5
Oliver, 195, 86.1
Tobias, 182, 90
Casper, 170, 56
";

    let separator = ',';

    let rows: Vec<std::collections::HashMap<String, CsvValue>> =
        csvvy::parse_csv(&input, separator);

    for row in rows {
        match row.get("height") {
            Some(CsvValue::Float(num)) => {
                // Do something
            }

            Some(CsvValue::Integer(num)) => {
                // Do something else
            }

            Some(CsvValue::Text(_)) | None => {
                // ignore
            }
        };
    }
}
```
