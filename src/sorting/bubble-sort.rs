fn bubble_sort(arr: &mut [i32]) -> Vec<i32> {
    if arr.len() <= 1 {
        return arr.to_vec();
    }

    for i in 0..arr.len()-1 {
        let mut swapped = false;

        for j in  1 ..arr.len()-i {
            if arr[j - 1] > arr[j] {
                arr.swap(j - 1, j);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }

    }

    arr.to_vec();
}