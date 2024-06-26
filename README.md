# Sorting Library in Rust

This is a Rust library which can do sorting of any object types.
It consists of 
-Insert sort;
-Select sort;
-Quick sort;
-Merge sort.

## How to install
Clone the repository:
```
git clone https://github.com/dayanabbb/sorting_rust.git
```
![rep](img/1.png)

## Usage

This library allows you to perform sorting operations on arrays using different sorting algorithms. 
You can use it in your Rust projects by adding it as a dependency in your `Cargo.toml` file:

```toml
[dependencies]
sorting = {path="path/to/sorting_rust"}
```
![dep](img/2.png)


## Then, import the sorting library:
```
use sorting::sorting::*;
use sorting::IntComparator;

```
![dep](img/4.png)

## Example
```
use sorting::sorting::*;
use sorting::IntComparator;

fn main() {
    let mut nums = vec![30, 10, 50, 20, 40];
    
    // Use insertion sort from the sorting_rust library
    insert_sort(&mut nums, &IntComparator);
    println!("Sorted numbers: {:?}", nums);
}
```
![dep](img/3.png)

![dep](img/5.png)
