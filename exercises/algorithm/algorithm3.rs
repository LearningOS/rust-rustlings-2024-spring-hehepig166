/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/


fn sort<T: Ord>(array: &mut [T]){
	//TODO
    // array.sort();
    if array.len() <= 1 {
        return;
    }
    
    let pivot = 0;
    let mut i = 0;
    let mut j = array.len() - 1;

    while i < j {
        while i < j && array[j] > array[pivot] {
            j -= 1;
        }
        while i < j && array[i] <= array[pivot] {
            i += 1;
        }

        if i != j {
            array.swap(i, j);
        }
    }
    array.swap(pivot, i);

    sort(&mut array[..i]);
    sort(&mut array[i+1..]);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}