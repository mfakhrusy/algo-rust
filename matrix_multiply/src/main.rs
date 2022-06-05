fn main() {
    let mat_a = vec![vec![1, 2], vec![3, 4]];
    let mat_b = vec![vec![1, 2], vec![3, 4]];
    assert_eq!(
        matrix_multiply(mat_a, mat_b),
        vec![vec![7, 10], vec![15, 22]]
    );

    let mat_a = vec![vec![1, -1], vec![0, 1]];
    let mat_b = vec![vec![1, -1], vec![0, 1]];
    assert_eq!(matrix_multiply(mat_a, mat_b), vec![vec![1, -2], vec![0, 1]]);

    let mat_a = vec![vec![2, -2]];
    let mat_b = vec![vec![5], vec![0]];
    assert_eq!(matrix_multiply(mat_a, mat_b), vec![vec![10]]);

    let mat_a = vec![vec![-100], vec![20], vec![0]];
    let mat_b = vec![vec![5, 4, -5]];
    assert_eq!(
        matrix_multiply(mat_a, mat_b),
        vec![vec![-500, -400, 500], vec![100, 80, -100], vec![0, 0, 0]]
    );

    let mat_a = vec![
        vec![1, 3, 0],
        vec![-1, -2, -5],
        vec![1, 0, 1],
        vec![3, 4, 5],
        vec![0, 0, 999],
    ];
    let mat_b = vec![
        vec![-12, 12, -12, 12, 0],
        vec![9, 8, 7, -6, -5],
        vec![4, 3, 2, 1, -10],
    ];
    assert_eq!(
        matrix_multiply(mat_a, mat_b),
        vec![
            vec![15, 36, 9, -6, -15],
            vec![-26, -43, -12, -5, 60],
            vec![-8, 15, -10, 13, -10],
            vec![20, 83, 2, 17, -70],
            vec![3996, 2997, 1998, 999, -9990]
        ]
    )
}

fn matrix_multiply(mat_a: Vec<Vec<i32>>, mat_b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if mat_a.len() != mat_b[0].len() {
        panic!("matrix can't be multiplicated")
    }

    let row_len = mat_a.len();
    let col_len = mat_b[0].len();

    let mut result = vec![vec![0; col_len]; row_len];

    for i in 0..row_len {
        for j in 0..col_len {
            let mut sum = 0;
            for k in 0..mat_a[0].len() {
                sum += mat_a[i][k] * mat_b[k][j];
            }
            result[i][j] = sum;
        }
    }

    return result;
}
