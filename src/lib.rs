pub trait Comparator<T> {
    fn compare(&self, a: &T, b: &T) -> bool;
}

pub mod sorting {
    use super::Comparator;

    pub fn insert_sort<T, C>(arr: &mut [T], comparator: &C)
    where
        C: Comparator<T>,
    {
        for i in 1..arr.len() {
            let mut j = i;
            while j > 0 && comparator.compare(&arr[j], &arr[j - 1]) {
                arr.swap(j, j - 1);
                j -= 1;
            }
        }
    }

    pub fn select_sort<T, C>(arr: &mut [T], comparator: &C)
    where
        C: Comparator<T>,
    {
        for i in 0..arr.len() {
            let mut min_index = i;
            for j in i + 1..arr.len() {
                if comparator.compare(&arr[j], &arr[min_index]) {
                    min_index = j;
                }
            }
            arr.swap(i, min_index);
        }
    }

    pub fn quick_sort<T, C>(arr: &mut [T], comparator: &C)
    where
        C: Comparator<T>,
    {
        if arr.len() <= 1 {
            return;
        }
        let pivot_index = partition(arr, comparator);
        quick_sort(&mut arr[0..pivot_index], comparator);
        quick_sort(&mut arr[pivot_index + 1..], comparator);
    }

    fn partition<T, C>(arr: &mut [T], comparator: &C) -> usize
    where
        C: Comparator<T>,
    {
        let pivot_index = arr.len() / 2;
        arr.swap(pivot_index, arr.len() - 1);
        let mut i = 0;
        for j in 0..arr.len() - 1 {
            if comparator.compare(&arr[j], &arr[arr.len() - 1]) {
                arr.swap(i, j);
                i += 1;
            }
        }
        arr.swap(i, arr.len() - 1);
        i
    }

    pub fn merge_sort<T, C>(arr: &mut [T], comparator: &C)
    where
        T: Copy,
        C: Comparator<T>,
    {
        if arr.len() <= 1 {
            return;
        }
        let mid = arr.len() / 2;
        merge_sort(&mut arr[..mid], comparator);
        merge_sort(&mut arr[mid..], comparator);
        let mut result = Vec::new();
        let (mut i, mut j) = (0, mid);
        while i < mid && j < arr.len() {
            if comparator.compare(&arr[i], &arr[j]) {
                result.push(arr[i].clone());
                i += 1;
            } else {
                result.push(arr[j].clone());
                j += 1;
            }
        }
        result.extend_from_slice(&arr[i..mid]);
        result.extend_from_slice(&arr[j..]);
        arr.copy_from_slice(&result);
    }
}

pub struct IntComparator;

impl Comparator<i32> for IntComparator {
    fn compare(&self, a: &i32, b: &i32) -> bool {
        a < b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! sorting_test {
        ($func:ident, $arr:expr) => {
            let mut nums = $arr;
            sorting::$func(&mut nums, &IntComparator);
            assert_eq!(nums, vec![10, 20, 30, 40, 50]);
        };
    }

    #[test]
    fn test_insert_sort() {
        sorting_test!(insert_sort, vec![30, 10, 50, 20, 40]);
    }

    #[test]
    fn test_select_sort() {
        sorting_test!(select_sort, vec![30, 10, 50, 20, 40]);
    }

    #[test]
    fn test_quick_sort() {
        sorting_test!(quick_sort, vec![30, 10, 50, 20, 40]);
    }

    #[test]
    fn test_merge_sort() {
        sorting_test!(merge_sort, vec![30, 10, 50, 20, 40]);
    }
}
