use std::{collections::HashSet, fs};

pub fn has_subset_sum(set: &[i64], sum: i64) -> bool {
    if sum == 0 {
        return true;
    }
    if sum < 0 || set.is_empty() {
        return false;
    }

    let with = has_subset_sum(&set[..set.len() - 1], sum - set[set.len() - 1]);
    let without = has_subset_sum(&set[..set.len() - 1], sum);

    return with || without;
}

pub fn count_subset_sum(set: &[i64], sum: i64) -> u64 {
    if sum == 0 {
        return 1;
    }
    if sum < 0 || set.is_empty() {
        return 0;
    }

    let with = count_subset_sum(&set[..set.len() - 1], sum - set[set.len() - 1]);
    let without = count_subset_sum(&set[..set.len() - 1], sum);

    return with + without;
}

fn _max_weight_subset_sum(set: &[i64], weights: &[i64], sum: i64, weight_sum: i64) -> i64 {
    if sum == 0 {
        return weight_sum;
    }
    if sum < 0 || set.is_empty() {
        return i64::MIN;
    }

    let with = _max_weight_subset_sum(
        &set[..set.len() - 1],
        &weights[..set.len() - 1],
        sum - set[set.len() - 1],
        weight_sum + weights[set.len() - 1],
    );

    let without = _max_weight_subset_sum(
        &set[..set.len() - 1],
        &weights[..set.len() - 1],
        sum,
        weight_sum,
    );

    return i64::max(with, without);
}

pub fn max_weight_subset_sum(set: &[i64], weights: &[i64], sum: i64) -> i64 {
    return _max_weight_subset_sum(set, weights, sum, 0);
}

struct IsWord {
    words: HashSet<String>,
}

impl IsWord {
    fn new() -> Self {
        let mut words = HashSet::new();

        for line in fs::read_to_string("/usr/share/dict/american-english")
            .unwrap()
            .lines()
        {
            if line.len() == 1 {
                continue;
            }
            words.insert(line.to_string());
        }

        Self { words }
    }

    fn is_word(self: &Self, word: &str) -> bool {
        self.words.contains(word)
    }
}

fn _text_segmentation_count(is_word: &IsWord, text: &str, count: u64) -> u64 {
    if text.is_empty() {
        return 1;
    }

    let mut count = count;

    for i in 0..text.len() + 1 {
        if is_word.is_word(&text[0..i]) {
            count += _text_segmentation_count(is_word, &text[i..], count);
        }
    }

    return count;
}

pub fn text_segmentation_count(text: &str) -> u64 {
    let is_word = IsWord::new();
    _text_segmentation_count(&is_word, text, 0) - 1
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_subset_sum_test() {
        assert_eq!(has_subset_sum(&[], 0), true);
        assert_eq!(has_subset_sum(&[], 1), false);
        assert_eq!(has_subset_sum(&[8, 6, 7, 5, 3, 10, 9], 15), true); // 8,7 7,5,3 6,9 5,10
        assert_eq!(has_subset_sum(&[11, 6, 5, 1, 7, 13, 12], 15), false);
    }
    #[test]
    fn count_subset_sum_test() {
        assert_eq!(count_subset_sum(&[], 0), 1);
        assert_eq!(count_subset_sum(&[], 1), 0);
        assert_eq!(count_subset_sum(&[8, 6, 7, 5, 3, 10, 9], 15), 4); // 8,7 7,5,3 6,9 5,10
        assert_eq!(count_subset_sum(&[11, 6, 5, 1, 7, 13, 12], 15), 0);
    }
    #[test]
    fn max_weight_subset_sum_test() {
        assert_eq!(max_weight_subset_sum(&[], &[], 0), 0);
        assert_eq!(max_weight_subset_sum(&[], &[], 1), i64::MIN);
        assert_eq!(
            max_weight_subset_sum(&[8, 6, 7, 5, 3, 10, 9], &[3, 2, 1, 10, 7, 20, 2], 15),
            30
        ); // 8,7 7,5,3 6,9 5,10
        assert_eq!(
            max_weight_subset_sum(&[11, 6, 5, 1, 7, 13, 12], &[1, 1, 1, 1, 1, 1, 1], 15),
            i64::MIN
        );
    }
    #[test]
    fn is_word_test() {
        let is_word = IsWord::new();
        assert_eq!(is_word.is_word("buffalo"), true);
        assert_eq!(is_word.is_word("artist"), true);
        assert_eq!(is_word.is_word("is"), true);
        assert_eq!(is_word.is_word("toil"), true);
        assert_eq!(is_word.is_word("oil"), true);
    }

    #[test]
    fn text_segmentation_count_test() {
        assert_eq!(text_segmentation_count(""), 0);
        assert_eq!(text_segmentation_count("artistoil"), 2);
    }
}
