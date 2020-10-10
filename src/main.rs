pub fn bubble_sort(arr: &mut std::vec::Vec<i32>) {
    let len = arr.len();
    let mut i = 0;

    while (i + 1) < len {
        let mut j = i;

        while (j + 1) < len {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
            j += 1;
        }
        i += 1;
    }
}

fn main() {
    let mut nums = vec![1, 4, 2, 3, 5];
    bubble_sort(&mut nums);
    println!("{:?}", nums);
}
