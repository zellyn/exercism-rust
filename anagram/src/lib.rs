use std::collections::HashSet;
use unicode_segmentation::UnicodeSegmentation;

fn letter_sort(word: &str) -> String {
    let mut graphemes = word.graphemes(true).collect::<Vec<&str>>();
    graphemes.sort_unstable();
    graphemes.into_iter().collect()
}

pub fn anagrams_for<'a>(target_word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let target_lowered = target_word.to_lowercase();
    let target_sorted = letter_sort(&target_lowered);
    possible_anagrams
        .iter()
        .copied()
        .filter(|x| {
            let lowered = x.to_lowercase();
            (lowered != target_lowered) && (letter_sort(&lowered) == target_sorted)
        })
        .collect()
}
