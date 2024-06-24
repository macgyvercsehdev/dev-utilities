# dev-utilities

`dev-utilities` is a Rust library created to help developers with utilities that optimize time and facilitate common everyday tasks. The library contains various useful functions that allow for efficient and intuitive data manipulation and transformation.

## Features

### remove_mask

Removes the specified mask characters from a string.

This function takes an input string `input` and a mask string `mask`, and returns a new string with characters from the mask removed.

#### Examples

```rust
use dev_utilities::utils::string_utils::remove_mask;

let result = remove_mask("123-456-789", "-");
assert_eq!(result, "123456789");

let result = remove_mask("(123) 456-7890", "()- ");
assert_eq!(result, "1234567890");
```

## Definition

```rust
pub fn remove_mask(input: &str, mask: &str) -> String {
    let mut result = String::with_capacity(input.len());
    for ch in input.chars() {
        if !mask.contains(ch) {
            result.push(ch);
        }
    }
    result
}

```

## Installation

To add dev-utilities to your project, add the following line to your Cargo.toml

```toml
[dependencies]
dev-utilities = "0.1.0"
```

## Contribution

Contributions are welcome! If you have suggestions, issues, or want to add new features, feel free to open an issue or a pull request on the GitHub repository.

