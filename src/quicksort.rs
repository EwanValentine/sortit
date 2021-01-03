pub struct QuickSort;

use super::Sorter;

fn partition<T>(v: &mut [T]) -> usize 
    where
        T: Ord,
{
    let len = v.len();
    let pivot = len / 2;
    let last = len - 1;

    v.swap(pivot, last);
    let mut store_index = 0;

    for i in 0..last {
        if &v[i] < &v[last] {
            v.swap(i, store_index);
            store_index += 1;
        }
    }

    v.swap(store_index, len - 1);

    store_index
}

impl Sorter for QuickSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        let len = slice.len();
        if len >= 2 {
            let pivot = partition(slice);
            self.sort(&mut slice[0..pivot]);
            self.sort(&mut slice[pivot + 1..len]);
        }
    }
}

#[test]
fn it_works() {
    let mut things = vec![4, 2, 5, 3, 1];
    QuickSort.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}
