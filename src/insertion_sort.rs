use std::{cmp, fmt::Debug};

pub fn insertion_sort<T>(arr: &[T]) -> Vec<T>
where
    T: cmp::PartialEq + cmp::PartialOrd + Clone + Debug,
{
    let mut result: Vec<T> = Vec::with_capacity(arr.len());//the same len
    for elem in arr.iter().cloned() {
        let n_inserted = result.len();
        for i in 0..=n_inserted {
            if i == n_inserted || result[i] > elem {
                result.insert(i, elem);
                break;
            }
        }
        println!("{:?}",result);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let res = insertion_sort(&Vec::<u8>::new());
        assert_eq!(res, vec![]);
    }

    #[test]
    fn one_element() {
        let res = insertion_sort(&vec!["a"]);
        assert_eq!(res, vec!["a"]);
    }

    #[test]
    fn already_sorted() {
        let res = insertion_sort(&vec!["a", "b", "c"]);
        assert_eq!(res, vec!["a", "b", "c"]);
    }

    #[test]
    fn basic() {
        let res = insertion_sort(&vec!["d", "a", "c", "b"]);
        assert_eq!(res, vec!["a", "b", "c", "d"]);
    }

    #[test]
    fn odd_number_of_elements() {
        let res = insertion_sort(&vec!["d", "a", "c", "e", "b"]);
        assert_eq!(res, vec!["a", "b", "c", "d", "e"]);
    }

    #[test]
    fn repeated_elements() {
        let res = insertion_sort(&vec![542, 542, 542, 542]);
        assert_eq!(res, vec![542, 542, 542, 542]);
    }
}
