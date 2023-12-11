fn main() {
    let mut data = vec![12, 4, 5, 6, 7, 3, 1, 15];

    println!("Original Data: {:?}", data);

    // Call the quicksort function to sort the array in-place.
    quicksort(&mut data);

    println!("Sorted Data: {:?}", data);
}

// Quicksort function
fn quicksort(data: &mut [i32]) {
    // Perform the quicksort algorithm on the mutable data
    quicksort_recursive(data, 0, (data.len() - 1) as isize);
}

// Recursive helper function for quicksort
fn quicksort_recursive(data: &mut [i32], low: isize, high: isize) {
    // Base case: If the low index is greater than or equal to the high index,
    // the subarray is already sorted, and we can return.
    if low >= high {
        return;
    }

    // Choose a pivot element.
    let pivot_index = partition(data, low, high);

    // Recursively sort the left and right subarrays.
    quicksort_recursive(data, low, pivot_index - 1);
    quicksort_recursive(data, pivot_index + 1, high);
}

// Partition function for quicksort
fn partition(data: &mut [i32], low: isize, high: isize) -> isize {
    // Choose the last element as the pivot
    let pivot = data[high as usize];
    let mut i = low - 1;

    // Loop through the subarray (excluding the pivot element)
    for j in low..high {
        // If the current element is less than or equal to the pivot,
        // swap it with the element at the next available position (i + 1).
        if data[j as usize] <= pivot {
            i += 1;
            data.swap(i as usize, j as usize);
        }
    }

    // Swap the pivot element with the element at position (i + 1),
    // which now separates the two subarrays.
    data.swap((i + 1) as usize, high as usize);

    // Return the final position of the pivot element.
    i + 1
}
