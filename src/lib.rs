pub mod backtracking;
pub mod binary_tree;
pub mod sort;

fn _binary_search_recursive<T: PartialOrd + std::fmt::Debug>(
    haystack: &[T],
    needle: T,
    from: usize,
    to: usize,
) -> Option<usize> {
    if from > to {
        return None;
    }
    let middle = from + (to - from) / 2;
    println!(
        "Haystack: {:?}, needle: {:?}, from: {:?}, to: {:?}, middle: {:?}",
        &haystack[from..=to],
        needle,
        from,
        to,
        middle
    );

    if haystack[middle] == needle {
        return Some(middle);
    } else if haystack[middle] > needle {
        return _binary_search_recursive(haystack, needle, from, middle - 1);
    } else {
        return _binary_search_recursive(haystack, needle, middle + 1, to);
    }
}

pub fn binary_search_recursive<T: PartialOrd + std::fmt::Debug>(
    haystack: &[T],
    needle: T,
) -> Option<usize> {
    if haystack.len() == 0 {
        return None;
    }
    return _binary_search_recursive(haystack, needle, 0, haystack.len() - 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        assert_eq!(binary_search_recursive(&[], 3), None);
        assert_eq!(binary_search_recursive(&[1, 2, 3], 4), None);
        assert_eq!(binary_search_recursive(&[1, 2, 3, 4], 4), Some(3));
        assert_eq!(binary_search_recursive(&[1, 2, 3, 4, 5], 4), Some(3));
    }
}
