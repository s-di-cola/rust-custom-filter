# Custom Filtering Function in Rust

A simple Rust program demonstrating custom filtering of collections using structs and methods. Filters elements from a
vector based on a specific condition without using closures.

## What it does

- Creates a filtering system using structs and methods
- Filters elements from a collection based on a custom condition
- Returns a new collection containing only matching elements
- Demonstrates basic Rust concepts like structs, methods, and iterators

## How to run

```bash
cargo run
```

## Example output

```
Original numbers: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
Filtered numbers: [2, 4, 6, 8, 10]
```

## Key concepts shown

- **Structs**: Defining custom data types (`FilterCondition`)
- **Methods**: Implementing behavior on structs (`is_match` method)
- **Functions**: Creating reusable filtering logic (`custom_filter` function)
- **Iterators**: Processing collections with `iter()` and `filter()`
- **References**: Using `&` to borrow data without taking ownership
- **Collections**: Working with `Vec<T>` and filtering operations

## Code structure

- `FilterCondition` struct - holds the filtering criteria
- `is_match` method - determines if an element matches the condition
- `custom_filter` function - applies the filter to a collection
- Main function - demonstrates usage with example data

## Learning objectives

- Understanding struct definition and method implementation
- Learning about collection filtering without closures
- Practicing reference usage and borrowing
- Building reusable filtering components

## Files

- `src/main.rs` - Complete filtering implementation with example usage