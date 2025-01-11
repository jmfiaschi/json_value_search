# json_value_search

[![Actions Status](https://github.com/jmfiaschi/json_value_search/workflows/CI/badge.svg)](https://github.com/jmfiaschi/json_value_search/actions/workflows/ci.yml)
[![semantic-release](https://img.shields.io/badge/%20%20%F0%9F%93%A6%F0%9F%9A%80-semantic--release-e10079.svg)](https://github.com/semantic-release/semantic-release)

Give an interface to search values into json_serde::Value.

# Getting Started

## Installation

Add the following line to your Cargo.toml:
```toml
[dependencies]
env_applier = "x.y.z" # Replace with the latest version
```

## Quick Start

### Basic Example

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

---

## Useful link

* [Benchmark report](https://jmfiaschi.github.io/json_value_search/bench/main/)
* [Package](https://crates.io/crates/json_value_search)

---

## Contributing

Contributions are welcome!

To contribute:

1. Fork the repository and create your branch (git checkout -b feature/my-feature).
2. Commit your changes (git commit -m 'Add some feature').
3. Push to the branch (git push origin feature/my-feature).
4. Open a pull request.

For major changes, please open an issue first to discuss your proposal.

Please ensure that tests are added or updated as appropriate.

---

## License

Licensed under either of the following, at your option:

* [Apache License 2.0](https://choosealicense.com/licenses/apache-2.0/)
* [MIT License](https://choosealicense.com/licenses/mit/)
