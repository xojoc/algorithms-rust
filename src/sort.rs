fn quicksort_partition<T: PartialOrd>(
    elements: &mut [T],
    from: usize,
    to: usize,
    pivot: usize,
) -> usize {
    elements.swap(pivot, to - 1);

    let mut l = 0;
    for i in from..to - 1 {
        if elements[from + i] <= elements[to - 1] {
            elements.swap(from + l, from + i);
            l += 1;
        }
    }
    elements.swap(from + l, to - 1);
    return from + l;
}
pub fn _quicksort_recursive<T: PartialOrd>(elements: &mut [T], from: usize, to: usize) {
    if from == to {
        return;
    }
    let pivot = from + (to - 1 - from) / 2;
    let split = quicksort_partition(elements, from, to, pivot);

    _quicksort_recursive(elements, from, split);
    _quicksort_recursive(elements, split + 1, to);
}
pub fn quicksort_recursive<T: PartialOrd>(elements: &mut [T]) {
    _quicksort_recursive(elements, 0, elements.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        let mut elements: Vec<usize> = vec![];
        quicksort_recursive(&mut elements);
        assert_eq!(elements, &[]);

        elements = vec![3, 2, 1];
        quicksort_recursive(&mut elements);
        assert_eq!(elements, &[1, 2, 3]);

        elements = vec![1, 3, 4, 5, 2, 1, 3, 7];
        quicksort_recursive(&mut elements);
        assert_eq!(elements, &[1, 1, 2, 3, 3, 4, 5, 7]);
    }
}
