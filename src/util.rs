pub(crate) fn transpose_vec<Item>(elements: &Vec<Vec<Item>>) -> Vec<Vec<Item>>
where
    Item: Copy,
{
    let capacity = elements.first().unwrap().len();
    let image_count = elements.len();

    let mut transformed: Vec<Vec<Item>> = vec![Vec::<Item>::with_capacity(image_count); capacity];

    for element in elements.iter() {
        for (inner_index, item) in element.iter().enumerate() {
            transformed[inner_index].push(*item);
        }
    }

    transformed
}
