fn prepare_matrices<F>(
    matrix1: &Vec<Vec<f64>>,
    matrix2: &Vec<Vec<f64>>,
    operator_func: F,
) -> Vec<Vec<f64>>
where
    F: Fn(&f64, &f64) -> f64,
{
    let size_of_matrix = matrix1.len();
    let size_of_vector_in_matrix = matrix1[0].len();
    let mut added_matrix = vec![vec![0.0; size_of_vector_in_matrix]; size_of_matrix];

    let mut broadcasted_matrix2 = matrix2.clone();

    println!("The length of matrix2 is {}", matrix2.len());
    if matrix2.len() != size_of_matrix && matrix2.len() == 1 {
        broadcasted_matrix2.resize(size_of_matrix, (*broadcasted_matrix2[0]).to_owned());
        println!("The broadcasted_matrix 2 is {:?}", broadcasted_matrix2);
    }

    for ((i, vector_matrix1), vector_matrix2) in
        matrix1.iter().enumerate().zip(&broadcasted_matrix2)
    {
        for ((j, element_vector_matrix1), element_vector_matrix2) in
            vector_matrix1.iter().enumerate().zip(vector_matrix2)
        {
            added_matrix[i][j] = operator_func(element_vector_matrix1, element_vector_matrix2)
        }
    }
    added_matrix
}
fn add_elements(value1: &f64, value2: &f64) -> f64 {
    value1 + value2
}

fn subtract_elements(value1: &f64, value2: &f64) -> f64 {
    value1 - value2
}

fn subtract_matrices(matrix1: &Vec<Vec<f64>>, matrix2: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    prepare_matrices(matrix1, matrix2, subtract_elements)
}

fn add_matrices(matrix1: &Vec<Vec<f64>>, matrix2: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    prepare_matrices(matrix1, matrix2, add_elements)
}

#[test]
fn addition_of_matrices_works_correctly() {
    let matrix1 = vec![vec![1.0, 2.5, 3.2], vec![4.0, 5.0, 6.0]];
    let matrix2 = vec![vec![1.0, 2.0, 3.0], vec![1.0, 2.0, 3.0]];

    let expected = vec![vec![2.0, 4.5, 6.2], vec![5.0, 7.0, 9.0]];

    let result = add_matrices(&matrix1, &matrix2);
    assert_eq!(expected, result);
}

#[test]
fn unequal_matrix_sizes_are_broadcasted_and_added() {
    let matrix1 = vec![vec![1.0, 2.5, 3.2], vec![4.0, 5.0, 6.0]];
    let matrix2 = vec![vec![1.0, 2.0, 3.0]];

    let expected = vec![vec![2.0, 4.5, 6.2], vec![5.0, 7.0, 9.0]];

    let result = add_matrices(&matrix1, &matrix2);
    assert_eq!(expected, result);
}

#[test]
fn subtraction_of_matrices_works_correctly() {
    let matrix1 = vec![vec![1.0, 2.5, 3.2], vec![4.0, 5.0, 6.0]];
    let matrix2 = vec![vec![1.0, 2.0, 3.0], vec![1.0, 2.0, 3.0]];

    let expected = vec![vec![0.0, 0.5, 0.2], vec![3.0, 3.0, 3.0]];

    let result = subtract_matrices(&matrix1, &matrix2);
    assert_eq!(expected, result);
}
