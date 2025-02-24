# Rust Substring Finder

## Overview

This Rust project provides an optimized implementation of the "Find Substring" problem using a **sliding window approach with HashMaps**.

## Features

- 🚀 **Optimized**: Fast execution using efficient memory management.
- ✅ **Unit Tests**: Includes multiple test cases.
- 📜 **Well-documented**: Code is easy to read and understand.

## Usage

```rust
use substring_finder::Solution;

let s = "barfoothefoobarman".to_string();
let words = vec!["foo".to_string(), "bar".to_string()];
let result = Solution::find_substring(s, words);
assert_eq!(result, vec![0, 9]);
```

## Running Tests

```sh
cargo test
```

## Installation

Clone the repository and run:

```sh
cargo build
```

## License

MIT
