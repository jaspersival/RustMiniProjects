fn add_matrix(matrix1: &Vec<Vec<f64>>, matrix2: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let size_of_matrix = matrix1.len();
    let size_of_vector_in_matrix = matrix1[0].len();
    let mut added_matrix = vec![vec![0.0; size_of_vector_in_matrix]; size_of_matrix];

    for ((i, vector_matrix1), vector_matrix2) in matrix1.iter().enumerate().zip(matrix2) {
        for ((j, element_vector_matrix1), element_vector_matrix2) in
            vector_matrix1.iter().enumerate().zip(vector_matrix2)
        {
            added_matrix[i][j] = element_vector_matrix1 + element_vector_matrix2
        }
    }
    added_matrix
}

#[test]
fn addition_of_matrices_works_correctly() {
    let matrix1 = vec![vec![1.0, 2.5, 3.2], vec![4.0, 5.0, 6.0]];
    let matrix2 = vec![vec![1.0, 2.0, 3.0], vec![1.0, 2.0, 3.0]];

    let expected = vec![vec![2.0, 4.5, 6.2], vec![5.0, 7.0, 9.0]];

    let result = add_matrix(&matrix1, &matrix2);
    assert_eq!(expected, result);
}

#[test]
fn unequal_matrix_sizes_are_broadcasted_and_added() {
    let matrix1 = vec![vec![1.0, 2.5, 3.2], vec![4.0, 5.0, 6.0]];
    let matrix2 = vec![vec![1.0, 2.0, 3.0]];

    let expected = vec![vec![2.0, 4.5, 6.2], vec![5.0, 7.0, 9.0]];

    let result = add_matrix(&matrix1, &matrix2);
    assert_eq!(expected, result);
}
