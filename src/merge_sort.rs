fn merge_sort<T: Ord + Clone>(collection: Vec<T>) -> Vec<T> {
    if collection.len() <= 1 {
        return collection;
    }

    let middle = collection.len() / 2;
    let left = collection[..middle].to_vec();
    let right = collection[middle..].to_vec();

    let sorted_left = merge_sort(left);
    let sorted_right = merge_sort(right);

    let mut result = Vec::with_capacity(sorted_left.len() + sorted_right.len());

    let mut i = 0;
    let mut j = 0;

    while i < sorted_left.len() && j < sorted_right.len() {
        if sorted_left[i] <= sorted_right[j] {
            result.push(sorted_left[i].clone());
            i += 1;
        } else {
            result.push(sorted_right[j].clone());
            j += 1;
        }
    }

    while i < sorted_left.len() {
        result.push(sorted_left[i].clone());
        i += 1;
    }

    while j < sorted_right.len() {
        result.push(sorted_right[j].clone());
        j += 1;
    }

    result
}

fn main() {
    let numbers = vec![4, 3, 2, 1];
    let sorted = merge_sort(numbers);
    println!("{:?}", sorted);
}