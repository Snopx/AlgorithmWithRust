use std::cmp::Ordering;

fn bubble_sort<T: Ord>(arr: &mut [T]) {
    for x in 0..arr.len() {
        let mut is_sort = true;
        for y in 0..arr.len() - 1 - x {
            if arr[y] > arr[y + 1] {
                arr.swap(y, y + 1);
                is_sort = false;
            }
        }
        if is_sort {
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn descending() {
        //descending
        let mut ve1 = vec![6, 5, 4, 3, 2, 1];
        bubble_sort(&mut ve1);
        for i in 0..ve1.len() - 1 {
            assert!(ve1[i] <= ve1[i + 1]);
        }
    }

    #[test]
    fn ascending() {
        //pre-sorted
        let mut ve2 = vec![1, 2, 3, 4, 5, 6];
        bubble_sort(&mut ve2);
        for i in 0..ve2.len() - 1 {
            assert!(ve2[i] <= ve2[i + 1]);
        }
    }
}
