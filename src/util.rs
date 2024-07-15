//! Collection of utility functions for various use cases.

/// Transposes a 2D Vector (Vec<Vec<Item>>)
///
/// # Examples
/// ```ignore
/// let vec = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
/// let result = transpose_vec(&vec);
/// assert_eq!(result, vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]]);
/// ```
pub(crate) fn transpose_vec<Item>(elements: &[Vec<Item>]) -> Vec<Vec<Item>>
where
    Item: Copy,
{
    let capacity = elements.first().unwrap().len();
    let image_count = elements.len();

    let mut transformed: Vec<Vec<Item>> = vec![Vec::<Item>::with_capacity(image_count); capacity];

    for element in elements {
        for (inner_index, item) in element.iter().enumerate() {
            transformed[inner_index].push(*item);
        }
    }

    transformed
}
