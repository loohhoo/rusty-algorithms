// Linear Search Algorithm 
// modified from https://github.com/TheAlgorithms/Rust/blob/master/src/searching/linear_search.rs
// added comments to explain what is going on

use std::cmp::PartialEq;

// Permit any type, implementing partialeq
pub fn linear_search<T: PartialEq>(item: &T, array: &[T]) -> Option<usize> {

    // go through each item, from 0 to the end of the array
    for (i, index) in array.iter().enumerate() {
        // if we found the item, return it and quit looping
        if item == index {
            return Some(i);
        }
    }

    // we didn't find anything, so return none
    None
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    // unlike binary search, this will not have an underflow
    fn empty() {
        let search = linear_search(&1, &vec![]);
        assert_eq!(search, None);
    }

    #[test]
    fn test() {
        let search = linear_search(&1, &vec![1, 2, 3]);
        assert_eq!(search, Some(0));
        
        let search = linear_search(&"A", &vec!["A", "a", "B", "b"]);
        assert_eq!(search, Some(0));
    }
}
