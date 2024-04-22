pub fn insertion_sort<T: PartialOrd>(arr: &mut [T], ascending: bool) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && ((ascending && arr[j - 1] > arr[j]) || (!ascending && arr[j - 1] < arr[j])) {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

pub fn merge_sort<T: Ord + Clone>(arr: &mut [T], ascending: bool) {
    let len = arr.len();
    if len > 1 {
        let mid = len / 2;

        let mut left = arr[..mid].to_vec();
        let mut right = arr[mid..].to_vec();

        merge_sort(&mut left, ascending);
        merge_sort(&mut right, ascending);

        let mut i = 0;
        let mut j = 0;
        let mut k = 0;

        while i < left.len() && j < right.len() {
            let condition = if ascending { left[i] < right[j] } else { left[i] > right[j] };
            if condition {
                arr[k] = left[i].clone();
                i += 1;
            } else {
                arr[k] = right[j].clone();
                j += 1;
            }
            k += 1;
        }

        while i < left.len() {
            arr[k] = left[i].clone();
            i += 1;
            k += 1;
        }

        while j < right.len() {
            arr[k] = right[j].clone();
            j += 1;
            k += 1;
        }
    }
}


pub fn quicksort<T: Ord>(arr: &mut [T], ascending: bool) {
    _quicksort(arr, 0, (arr.len() - 1) as isize, ascending);
}

fn _quicksort<T: Ord>(arr: &mut [T], left: isize, right: isize, ascending: bool) {
    if left < right {
        let partition_idx = partition(arr, left, right, ascending);

        _quicksort(arr, left, partition_idx - 1, ascending);
        _quicksort(arr, partition_idx + 1, right, ascending);
    }
}

fn partition<T: Ord>(arr: &mut [T], left: isize, right: isize, ascending: bool) -> isize {
    let pivot = right;
    let mut i: isize = left - 1;

    for j in left..right {
        if (ascending && arr[j as usize] <= arr[pivot as usize])
            || (!ascending && arr[j as usize] >= arr[pivot as usize])
        {
            i += 1;
            arr.swap(i as usize, j as usize);
        }
    }

    arr.swap((i + 1) as usize, pivot as usize);

    i + 1
}

pub fn selection_sort<T: Ord>(list: &mut [T], ascending: bool) {
    for i in 0..list.len() {
        let mut small = i;
        for j in (i + 1)..list.len() {
            if (ascending && list[j] < list[small]) || (!ascending && list[j] > list[small]) {
                small = j;
            }
        }
        list.swap(small, i);
    }
}
