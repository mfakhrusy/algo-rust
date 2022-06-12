use std::fs;

fn main() {
    let mut data = read_data("count_inversion/test_files/big.txt");
    count_inversion(data);
}

fn read_data(path: &str) -> Vec<i32> {
    let raw =
        fs::read_to_string(path).expect("unable to read file");
    let data = raw
        .split("\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|s| s.parse::<i32>().expect("input is not a valid integer"))
        .collect::<Vec<i32>>();

    return data;
}

fn count_inversion(arr: Vec<i32>) -> u32 {
    let mut total_count = 0;

    sort(arr, &mut total_count);

    return total_count;
}

fn sort(arr: Vec<i32>, mut total_count: &mut u32) -> Vec<i32> {
    let len = arr.len();
    if len <= 1 {
        return arr;
    }

    let mid = len / 2;
    let (left, right) = arr.split_at(mid);
    let sorted_left_arr = sort(left.to_vec(), &mut total_count);
    let sorted_right_arr = sort(right.to_vec(), &mut total_count);

    let merged_array = merge(sorted_left_arr, sorted_right_arr, &mut total_count);

    return merged_array;
}

fn merge(left_arr: Vec<i32>, right_arr: Vec<i32>, mut total_count: &mut u32) -> Vec<i32> {
    let total_length = left_arr.len() + right_arr.len();
    let mut merged_array = vec![];
    let mut main_index = 0;
    let mut left_index = 0;
    let mut right_index = 0;


    while main_index < total_length {
        if right_arr[right_index] < left_arr[left_index] {
            merged_array.push(right_arr[right_index]);
            *total_count += (left_arr.len() as u32 - left_index as u32);
            right_index += 1;
        } else {
            merged_array.push(left_arr[left_index]);
            left_index += 1;
        }

        if right_index >= right_arr.len() {
            let mut rest_left_arr = left_arr[left_index..].to_vec();
            main_index += rest_left_arr.len();
            // *total_count += 1;
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

