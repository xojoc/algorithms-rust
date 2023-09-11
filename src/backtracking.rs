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

fn _addition_chains(
    n: u64,
    starting_from: u64,
    chain: &mut Vec<u64>,
    solutions: &mut Vec<Vec<u64>>,
) {
    if starting_from > n {
        if !solutions.contains(chain) {
            solutions.push(chain.clone());
        }
        return;
    }

    for x in starting_from..=n {
        for j in 0..chain.len() {
            for i in 0..=j {
                if chain[i] + chain[j] == x {
                    chain.push(x);
                    _addition_chains(n, x + 1, chain, solutions);
                    chain.pop();
                }
            }
        }
    }
}

pub fn addition_chains(n: u64) -> Vec<Vec<u64>> {
    if n == 0 {
        return vec![];
    }
    let mut chain = vec![1];
    let mut solutions = vec![];
    _addition_chains(n, 2, &mut chain, &mut solutions);
    // solutions.sort();
    return solutions;
}
fn _longest_common_subsequence(
    a: &[usize],
    ai: usize,
    b: &[usize],
    bi: usize,
    lcs: &mut Vec<usize>,
) {
    if ai >= a.len() || bi >= b.len() {
        return;
    }
    if a[ai] == b[bi] {
        lcs.push(a[ai]);
        _longest_common_subsequence(a, ai + 1, b, bi + 1, lcs);
    } else {
        let mut lena = vec![];
        let mut lenb = vec![];
        _longest_common_subsequence(a, ai + 1, b, bi, &mut lena);
        _longest_common_subsequence(a, ai, b, bi + 1, &mut lenb);

        if lena.len() > lenb.len() {
            lcs.resize(lena.len(), 0);
            lcs.copy_from_slice(&lena);
        } else {
            lcs.resize(lenb.len(), 0);
            lcs.copy_from_slice(&lenb);
        }
    }
}
pub fn longest_common_subsequence(a: &[usize], b: &[usize]) -> Vec<usize> {
    let mut lcs = vec![];

    _longest_common_subsequence(a, 0, b, 0, &mut lcs);

    return lcs;
}

fn _shortest_common_supersequence(
    a: &[u64],
    ai: usize,
    b: &[u64],
    bi: usize,
    scs: &mut Vec<u64>,
    current_sequence: &mut Vec<u64>,
) {
    if ai >= a.len() && bi >= b.len() {
        if scs.len() == 0 || current_sequence.len() < scs.len() {
            scs.resize(current_sequence.len(), 0);
            scs.copy_from_slice(&current_sequence);
        }
        return;
    }

    if ai >= a.len() || bi >= b.len() || a[ai] == b[bi] {
        current_sequence.push(if ai < a.len() { a[ai] } else { b[bi] });
        _shortest_common_supersequence(a, ai + 1, b, bi + 1, scs, current_sequence);
        current_sequence.pop();
    } else {
        current_sequence.push(a[ai]);
        _shortest_common_supersequence(a, ai + 1, b, bi, scs, current_sequence);
        current_sequence.pop();
        current_sequence.push(b[bi]);
        _shortest_common_supersequence(a, ai, b, bi + 1, scs, current_sequence);
        current_sequence.pop();
    }
}
pub fn shortest_common_supersequence(a: &[u64], b: &[u64]) -> Vec<u64> {
    let mut scs = vec![];
    let mut current_sequence = vec![];

    _shortest_common_supersequence(a, 0, b, 0, &mut scs, &mut current_sequence);

    return scs;
}

#[derive(PartialEq)]
enum Direction {
    Increasing,
    Decreasing,
}

fn _longest_bitonic_sequence(
    a: &[u64],
    ai: usize,
    direction: Direction,
    lbs_start: &mut usize,
    lbs_end: &mut usize,
    current_sequence_start: &mut usize,
    current_sequence_end: &mut usize,
) {
    if (*current_sequence_end - *current_sequence_start) > (*lbs_end - *lbs_start) {
        *lbs_start = *current_sequence_start;
        *lbs_end = *current_sequence_end;
    }
    if ai == a.len() {
        return;
    }

    if ai == 0
        || (direction == Direction::Increasing && a[ai - 1] < a[ai]
            || direction == Direction::Decreasing && a[ai - 1] > a[ai])
    {
        *current_sequence_end += 1;
        _longest_bitonic_sequence(
            a,
            ai + 1,
            direction,
            lbs_start,
            lbs_end,
            current_sequence_start,
            current_sequence_end,
        );
    } else {
        if direction == Direction::Increasing {
            *current_sequence_end += 1;
            _longest_bitonic_sequence(
                a,
                ai + 1,
                Direction::Decreasing,
                lbs_start,
                lbs_end,
                current_sequence_start,
                current_sequence_end,
            );
        } else {
            *current_sequence_start = ai;
            *current_sequence_end = *current_sequence_start + 1;
            _longest_bitonic_sequence(
                a,
                ai + 1,
                Direction::Increasing,
                lbs_start,
                lbs_end,
                current_sequence_start,
                current_sequence_end,
            );
        }
    }
}

pub fn longest_bitonic_sequence(a: &[u64]) -> &[u64] {
    let mut lbs_start = 0;
    let mut lbs_end = 0;
    let mut current_sequence_start = 0;
    let mut current_sequence_end = 0;
    _longest_bitonic_sequence(
        a,
        0,
        Direction::Increasing,
        &mut lbs_start,
        &mut lbs_end,
        &mut current_sequence_start,
        &mut current_sequence_end,
    );

    return &a[lbs_start..lbs_end];
}

pub fn longest_oscilating_sequence(a: &[u64]) -> &[u64] {
    let mut current_sequence_start = 0;
    let mut current_sequence_end = 0;
    let mut los_start = 0;
    let mut los_end = 0;

    _longest_oscilating_sequence(
        a,
        0,
        &mut current_sequence_start,
        &mut current_sequence_end,
        &mut los_start,
        &mut los_end,
    );

    return &a[los_start..los_end];
}

// todo: rewrite these functions using return values instead of pointers?
fn _longest_oscilating_sequence(
    a: &[u64],
    ai: usize,
    current_sequence_start: &mut usize,
    current_sequence_end: &mut usize,
    los_start: &mut usize,
    los_end: &mut usize,
) {
    if (*current_sequence_end - *current_sequence_start) > (*los_end - *los_start) {
        *los_start = *current_sequence_start;
        *los_end = *current_sequence_end;
    }
    if ai >= a.len() {
        return;
    }

    dbg!(
        a[ai],
        ai,
        *current_sequence_start,
        *current_sequence_end,
        *los_start,
        *los_end
    );
    dbg!();

    if ai == a.len() - 1 || (ai % 2 == 0 && a[ai] < a[ai + 1]) || (ai % 2 == 1 && a[ai] > a[ai + 1])
    {
        *current_sequence_end += 1;
        _longest_oscilating_sequence(
            a,
            ai + 1,
            current_sequence_start,
            current_sequence_end,
            los_start,
            los_end,
        );
    } else {
        *current_sequence_start = ai + 1;
        *current_sequence_end = ai + 1;
        _longest_oscilating_sequence(
            a,
            ai + 1,
            current_sequence_start,
            current_sequence_end,
            los_start,
            los_end,
        )
    }
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

    #[test]
    fn addition_chains_test() {
        assert_eq!(addition_chains(0), vec![] as Vec<Vec<u64>>);
        assert_eq!(addition_chains(2), vec![vec![1, 2]]);
        assert_eq!(
            addition_chains(5),
            vec![vec![1, 2, 3, 4, 5], vec![1, 2, 3, 5], vec![1, 2, 4, 5]]
        );
    }

    #[test]
    fn longest_common_subsequence_test() {
        assert_eq!(longest_common_subsequence(&[], &[]), &[]);
        assert_eq!(longest_common_subsequence(&[1], &[1]), &[1]);
        assert_eq!(longest_common_subsequence(&[1, 2, 1], &[2]), &[2]);
        assert_eq!(
            longest_common_subsequence(&[1, 2, 1, 5, 7, 8, 9], &[2, 1, 7, 8, 9]),
            &[7, 8, 9]
        );
    }

    #[test]
    fn shortest_common_supersequence_test() {
        assert_eq!(shortest_common_supersequence(&[], &[]), &[]);
        assert_eq!(shortest_common_supersequence(&[1], &[1]), &[1]);
        assert_eq!(shortest_common_supersequence(&[2], &[1, 2]), &[1, 2]);
        assert_eq!(
            shortest_common_supersequence(&[5, 6, 7, 8], &[6, 8, 9]),
            &[5, 6, 7, 8, 9]
        );
    }

    #[test]
    fn longest_bitonic_sequence_test() {
        assert_eq!(longest_bitonic_sequence(&[]), &[]);
        assert_eq!(longest_bitonic_sequence(&[1]), &[1]);
        assert_eq!(longest_bitonic_sequence(&[1, 2, 3, 2, 1]), &[1, 2, 3, 2, 1]);
        assert_eq!(
            longest_bitonic_sequence(&[1, 2, 3, 2, 1, 5, 6, 7, 8, 4, 2, 10]),
            &[5, 6, 7, 8, 4, 2]
        );
    }

    #[test]
    fn longest_oscilating_sequence_test() {
        assert_eq!(longest_oscilating_sequence(&[]), &[]);
        assert_eq!(longest_oscilating_sequence(&[1]), &[1]);
        assert_eq!(longest_oscilating_sequence(&[1, 2, 1]), &[1, 2, 1]);
        assert_eq!(
            longest_oscilating_sequence(&[1, 2, 1, 0, 10, 11, 10, 11, 10]),
            &[10, 11, 10, 11, 10]
        );
    }
}
