use std::cmp::Ordering;
pub fn binary_search<T: Ord>(item: &T, arr: &[T]) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;

        match item.cmp(&arr[mid]) {
            Ordering::Less => right = mid,
            Ordering::Equal => return Some(mid),
            Ordering::Greater => left = mid + 1,
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn search_strings() {
        let index = binary_search(&"a", &vec!["a", "b", "c", "d", "google", "zoo"]);
        println!("{:?}", index);
        assert_eq!(index, Some(0));
    }
}
