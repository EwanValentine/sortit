pub struct InsertionSort {
    smart: bool,
}

use super::Sorter;

impl Sorter for InsertionSort {
    fn sort<T>(&self, slice: &mut [T])
    where 
        T: Ord,
    {
        for unsorted in 1..slice.len() {
            // slice[unsorted..] is not sorted
            // take slice[unsorted] and place in sorted location up to slice[..=unsorted]
            // [ 1 3 4 | 2 ]
            // [ 1 3 4 2 | ]
            // [ 1 3 2 4 | ]
            // [ 1 2 3 4 | ]
            if !self.smart {
                let mut i = unsorted;

                // While the previous value in the slice
                // is less than the first value
                while i > 0 && slice[i - 1] > slice[i] {
                    slice.swap(i - 1, i);
                    i -= 1;
                }
            } else {
                let i = slice[..unsorted]
                    .binary_search(&slice[unsorted])
                    .unwrap_or_else(|i| i);
                slice[i..=unsorted].rotate_right(1);
            }
        }
    }
}

#[test]
fn test_std_sort() {
    let mut things = vec![4, 2, 3, 1];
    InsertionSort{smart: false}.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4]);
}

#[test]
fn test_smart_sort() {
    let mut things = vec![4, 2, 3, 1];
    InsertionSort{smart: true}.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4]);
}
