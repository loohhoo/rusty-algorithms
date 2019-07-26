// Binary Search Algorithm 
// modified from https://github.com/TheAlgorithms/Rust/blob/master/src/searching/binary_search.rs
// fixed error involving an underflow.
// added comments explaining what this is doing.

use std::cmp::{PartialEq, PartialOrd};

// Permit any type, implementing partialeq, partialord traits.
pub fn binary_search<T: PartialEq + PartialOrd>(item: &T, array: &[T]) -> Option<usize> {
    // the array must have at least 1 item, or an underflow occurs
    // so we return none.
    if array.len() < 1 {
        println!("Please use an array with at least 1 item.");
        None
    }

    else {
        // Step 1 of Binary Search: define the left and right sides
        let mut left = 0;
        let mut right = array.len() - 1;

        // Step 2: 
        // as long as left is not right, we will repeatedly redefine the middle until we find the item
        // if there is an inequality at the middle item to the search term, we eliminate
        // the appropriate half of the array, and then recalculate the middle.
        while left < right {
            let mid = left + (right - left) / 2;

            // Step 3:
            // if the item at the middle is more than the item we want, eliminate
            // the right side, as it must be smaller than that.
            if &array[mid] > &item {
                right = mid - 1;
            }

            // but if it is less than the item we want, eliminate the left side.
            else if &array[mid] < &item {
                left = mid + 1;
            }

            // Step 4: Repeat loop, until we get here.
            // this is the end of the search.
            else {
                left = mid;
                break;
            }
        }

        // Step 5:
        // return the index where we found the item
        if &array[left] == item {
            Some(left)
        }

        // we didn't find it - return nothing
        else {
            None
        }
    }
    
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_empty() {
        let index = binary_search(&1, &mut vec![]);
        assert_eq!(index, None);
    }

    #[test]
    fn search_strings() {
        let index = binary_search(&"a", &vec!["a", "b", "c", "d"]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn search_ints() {
        let index = binary_search(&1, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(0));

        let index = binary_search(&2, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(1));

        let index = binary_search(&5, &vec![1, 2, 3, 4]);
        assert_eq!(index, None);
    }
}
