# json_value_search

[![Actions Status](https://github.com/jmfiaschi/json_value_search/workflows/CI/badge.svg)](https://github.com/jmfiaschi/json_value_search/actions)

Give an interface to search values into json_serde::Value.

## Installation

 ```Toml
[dependencies]
json_value_search = "0.1"
```

## Usage

```rust
extern crate json_value_search;

use json_value_search::Search;
use serde_json::Value;

let value: Value = serde_json::from_str(r#"{"field_A":[{"field.B":"value_B"},{"field_C":"value_C"}]}"#).unwrap();

let value_expected_with_wildcard: Value = serde_json::from_str(r#"[{"field.B":"value_B"},{"field_C":"value_C"}]"#).unwrap();
assert_eq!(Some(value_expected_with_wildcard),value.clone().search(r#"/field_A/*"#));

let value_expected_for_specific_field: Value = serde_json::from_str(r#"["value_B"]"#).unwrap();
assert_eq!(Some(value_expected_for_specific_field),value.clone().search(r#"/field_A/*/field.B"#));

let value_expected_for_specific_index: Value = serde_json::from_str(r#"{"field.B":"value_B"}"#).unwrap();
assert_eq!(Some(value_expected_for_specific_index),value.clone().search(r#"/field_A/0"#));

let value_expected_with_regex: Value = serde_json::from_str(r#"["value_B","value_C"]"#).unwrap();
assert_eq!(Some(value_expected_with_regex),value.clone().search(r#"/field_A/*/field.+"#));
```

## Bench

```
Bench json_value.search(/field/other_field).                                                                             
                        time:   [33.771 us 33.857 us 33.950 us]
                        change: [-13.330% -10.288% -7.2919%] (p = 0.00 < 0.05)
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe

Bench json_value.search(/field/*/other_field).                                                                            
                        time:   [81.117 us 84.117 us 87.467 us]
                        change: [+3.4453% +6.8087% +10.315%] (p = 0.00 < 0.05)
Found 10 outliers among 100 measurements (10.00%)
  9 (9.00%) high mild
  1 (1.00%) high severe

Bench json_value.search(/field/1/other_field).                                                                             
                        time:   [45.043 us 46.249 us 47.626 us]
                        change: [-3.0050% +1.0537% +4.8742%] (p = 0.61 > 0.05)
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) high mild
  5 (5.00%) high severe

Bench json_value.search(/field/1).                                                                             
                        time:   [23.609 us 23.916 us 24.354 us]
                        change: [-1.9539% -0.8737% +0.3058%] (p = 0.13 > 0.05)
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe

Bench json_value.search(/field/*/regex).                                                                            
                        time:   [127.65 us 130.88 us 134.64 us]
                        change: [+6.0142% +8.2959% +10.611%] (p = 0.00 < 0.05)
Found 14 outliers among 100 measurements (14.00%)
  10 (10.00%) high mild
  4 (4.00%) high severe
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[Apache](https://choosealicense.com/licenses/apache-2.0/)
[MIT](https://choosealicense.com/licenses/mit/)
