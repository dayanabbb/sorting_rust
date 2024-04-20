# Sorting Library in Rust

This is a Rust library which can do sorting of any object types.
It consists of 
1)Insert sort;
2)Select sort;
3)Quick sort;
4)Merge sort.

## How to install
Clone the repository:
```
git clone https://github.com/dayanabbb/sorting_rust.git
```

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
## Examples
```
use sorting_library::sorting::*;

fn main() {
    let mut nums = vec![5, 2, 7, 1, 9];
    
    // Perform insertion sort
    insertion_sort(&mut nums);
    println!("Insertion Sort: {:?}", nums);

    // Perform selection sort
    selection_sort(&mut nums);
    println!("Selection Sort: {:?}", nums);

    // Perform quick sort
    quick_sort(&mut nums);
    println!("Quick Sort: {:?}", nums);

    // Perform merge sort
    merge_sort(&mut nums);
    println!("Merge Sort: {:?}", nums);
}
```
