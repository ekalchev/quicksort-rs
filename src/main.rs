fn main() {
    let mut array = [7, 3, 2, 5, 6, 12, 54, 66, 98, 45, 23, 1];
    let len = array.len() - 1;
    sort_array(&mut array, 0, len);
}

pub fn sort_array(array: &mut [i32], start_index: usize, end_index: usize) {
    let pivot = array[end_index / 2];
    let mut i = start_index;
    let mut j = end_index;

    while i <= j {
        while array[i] < pivot {
            i += 1;
        }

        while array[j] > pivot {
            j -= 1;
        }

        if i <= j {
            array.swap(i, j);

            if i < end_index {
                i += 1;
            }

            if j > start_index {
                j -= 1;
            }
        }
    }

    if start_index < j {
        sort_array(array, start_index, j);
    }

    if end_index > i {
        sort_array(array, i, end_index);
    }
}
