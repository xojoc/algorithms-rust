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

// todo: can we remove current_index and use pointers only?
fn _text_segmentation_same_indices(is_word: &IsWord, texts: &[&str], current_index: usize) -> bool {
    if texts
        .iter()
        .all(|t| current_index <= t.len() && t[current_index..].len() == 0)
    {
        return true;
    }

    for l in (current_index + 1)..=texts[0].len() {
        if !texts.iter().all(|t| is_word.is_word(&t[current_index..l])) {
            continue;
        }
        let ok = _text_segmentation_same_indices(is_word, texts, l);
        if ok {
            return true;
        }
    }

    return false;
}
pub fn text_segmentation_same_indices(texts: &[&str]) -> bool {
    if !texts.iter().all(|t| texts[0].len() == t.len()) {
        return false;
    }
    let is_word = IsWord::new();
    _text_segmentation_same_indices(&is_word, texts, 0)
}
fn _text_segmentation_same_indices_count(
    is_word: &IsWord,
    texts: &[&str],
    current_index: usize,
    count: &mut usize,
) {
    if texts
        .iter()
        .all(|t| current_index <= t.len() && t[current_index..].len() == 0)
    {
        *count = *count + 1;
        return;
    }

    for l in (current_index + 1)..=texts[0].len() {
        if !texts.iter().all(|t| is_word.is_word(&t[current_index..l])) {
            continue;
        }
        _text_segmentation_same_indices_count(is_word, texts, l, count);
    }
}
pub fn text_segmentation_same_indices_count(texts: &[&str]) -> usize {
    if !texts.iter().all(|t| texts[0].len() == t.len()) {
        return 0;
    }
    let is_word = IsWord::new();
    let mut count = 0;
    _text_segmentation_same_indices_count(&is_word, texts, 0, &mut count);
    return count;
}
fn _nqueens_list(rows: &mut [usize], current_row: usize, solutions: &mut Vec<Vec<usize>>) {
    if current_row == rows.len() {
        solutions.push(rows.to_vec());
        return;
    }

    let n = rows.len();

    for col in 0..n {
        rows[current_row] = col;
        let mut ok = true;

        for row in 0..current_row {
            // check if same column
            if rows[row] == col {
                ok = false;
                break;
            }

            // check if same diagonal (from left to rigth)
            if rows[row] + (current_row - row) == col {
                ok = false;
                break;
            }

            // check if same diagonal (from right to left)
            if rows[row] == col + (current_row - row) {
                ok = false;
                break;
            }
        }

        if !ok {
            continue;
        }

        _nqueens_list(rows, current_row + 1, solutions);
    }
}

pub fn nqueens_list(n: usize) -> Vec<Vec<usize>> {
    let mut rows = vec![0; n];
    let mut solutions: Vec<Vec<usize>> = vec![];

    _nqueens_list(&mut rows, 0, &mut solutions);
    return solutions;
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
        assert_eq!(is_word.is_word("bot"), true);
        assert_eq!(is_word.is_word("heart"), true);
        assert_eq!(is_word.is_word("hand"), true);
        assert_eq!(is_word.is_word("sat"), true);
        assert_eq!(is_word.is_word("urns"), true);
        assert_eq!(is_word.is_word("pin"), true);
        assert_eq!(is_word.is_word("start"), true);
        assert_eq!(is_word.is_word("raps"), true);
        assert_eq!(is_word.is_word("and"), true);
        assert_eq!(is_word.is_word("rags"), true);
        assert_eq!(is_word.is_word("lap"), true);
    }

    #[test]
    fn text_segmentation_count_test() {
        assert_eq!(text_segmentation_count(""), 0);
        assert_eq!(text_segmentation_count("artistoil"), 2);
    }

    #[test]
    fn text_segmentation_same_indices_test() {
        assert_eq!(text_segmentation_same_indices(&[]), true);
        assert_eq!(text_segmentation_same_indices(&[""]), true);
        assert_eq!(text_segmentation_same_indices(&["", ""]), true);
        assert_eq!(
            text_segmentation_same_indices(&["bothearthandsaturnspin", "pinstartrapsandragslap"]),
            true
        );

        assert!(!text_segmentation_same_indices(&["different", "length"]));
    }

    #[test]
    fn text_segmentation_same_indices_count_test() {
        assert_eq!(text_segmentation_same_indices_count(&[]), 1);
        assert_eq!(
            text_segmentation_same_indices_count(&["dograt", "catdog"]),
            1
        );
        assert_eq!(
            text_segmentation_same_indices_count(&["artistoil", "artistoil"]),
            2
        );
    }

    #[test]
    fn nqueens_test() {
        assert_eq!(nqueens_list(0), vec![vec![]]);
        assert_eq!(nqueens_list(1), vec![vec![0]]);
        assert_eq!(nqueens_list(2), vec![] as Vec<Vec<usize>>);
        assert_eq!(nqueens_list(4).len(), 2);
        assert_eq!(nqueens_list(8).len(), 92);
    }
}
