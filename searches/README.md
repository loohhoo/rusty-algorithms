# Search Algorithms

## Binary Search 

Binary search works by following the below steps
1. Set the left side to 0, and the right to the end of the array (array.len() - 1).
2. If left > right, stop the loop. Otherwise, continue.
3. Set the middle to left + (right - left) / 2.
4. If there is an inequality from the middle item and our search term, do the following:
  a. if the middle is greater than the item, eliminate the right side (right = mid - 1)
  b. if the middle is less than the item, eliminate the left side (left = mid + 1)
  c. if they are equal, we have reached the end of the array. So, set left to mid.
5. Repeat until condition 4.c is met.
6. If the array at the left equals the search term, return the index (left), otherwise return nothing.

### Pseudocode: 

```
binary_search(item, array) {
  if array.len() < 1 {
    print("Your array needs at least 1 item.");
    return -1;
  }
  
  else {
    left = 0;
    right = array.len() - 1;

    while left < right {
      let mid = left + (right - left) / 2;

      if array[mid] > item {
        right = mid -1;
      }

      else if array[mid] < item {
        left = mid + 1;
      }

      else {
        left = mid;
        break;
      }
    }

    if array[left] = item {
      return left;
    }

    else {
      return -1;
    }
  }
}
```

Binary Search operates at O(log n) time on average. It will also underflow if you do not check that the array provided has at least 1 member.

## Linear Search

Linear search is fairly self-explanatory; it searches linearly from start to the end and breaks the loop when it finds the item.

### Pseudocode:

```
linear_search(item, array) {
  for i, item in array.iter().enumerate() {
    if array[i] == item {
      return i;
    }
  }
  
  return -1;
}
