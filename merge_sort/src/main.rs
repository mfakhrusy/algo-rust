fn main() {
    assert_eq!(merge_sort(vec![1, 5, 3]), vec![1, 3, 5]);
    assert_eq!(merge_sort(vec![1]), vec![1]);
    assert_eq!(merge_sort(vec![]), vec![]);
    assert_eq!(merge_sort(vec![5, 4, 3, 2, 1]), vec![1, 2, 3, 4, 5]);
    assert_eq!(
        merge_sort(vec![5, 4, 3, 2, 1, 2, 3, 4, 5]),
        vec![1, 2, 2, 3, 3, 4, 4, 5, 5]
    );
    assert_ne!(
        merge_sort(vec![5, 4, 3, 2, 1, 2, 3, 4, 5, 6]),
        vec![1, 2, 2, 3, 3, 4, 4, 5, 5]
    );
}

fn merge_sort(arr: Vec<i32>) -> Vec<i32> {
    let len = arr.len();
    if len <= 1 {
        return arr;
    }

    let mid = len / 2;
    let (left, right) = arr.split_at(mid);
    let sorted_left_arr = merge_sort(left.to_vec());
    let sorted_right_arr = merge_sort(right.to_vec());

    let merged_array = merge(sorted_left_arr, sorted_right_arr);

    return merged_array;
}

fn merge(left_arr: Vec<i32>, right_arr: Vec<i32>) -> Vec<i32> {
    let total_length = left_arr.len() + right_arr.len();
    let mut merged_array = vec![];
    let mut main_index = 0;
    let mut left_index = 0;
    let mut right_index = 0;

    while main_index < total_length {
        if left_arr[left_index] >= right_arr[right_index] {
            merged_array.push(right_arr[right_index]);
            right_index += 1;
        } else {
            merged_array.push(left_arr[left_index]);
            left_index += 1;
        }

        if right_index >= right_arr.len() {
            let mut rest_left_arr = left_arr[left_index..].to_vec();
            main_index += rest_left_arr.len();
            merged_array.append(&mut rest_left_arr);
        } else if left_index >= left_arr.len() {
            let mut rest_right_arr: Vec<i32> = right_arr[right_index..].to_vec();
            main_index += rest_right_arr.len();
            merged_array.append(&mut rest_right_arr);
        }

        main_index += 1;
    }

    return merged_array;
}
