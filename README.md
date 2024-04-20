# Sorting Library in Rust

This is a Rust library which can do sorting of any object types.

## Usage

This library allows you to perform sorting operations on arrays using different sorting algorithms. 
You can use it in your Rust projects by adding it as a dependency in your `Cargo.toml` file:

```toml
[dependencies]
sorting_library = "0.1.0"
```

## Then, you can use the sorting functions in your Rust code as follows:
```
use sorting_library::sorting::*;

fn main() {
    let mut nums = vec![5, 2, 7, 1, 9];
    insertion_sort(&mut nums);
    println!("Sorted numbers: {:?}", nums);
}
```
