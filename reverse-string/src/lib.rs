use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut clusters = input.graphemes(true).collect::<Vec<&str>>();
    clusters.reverse();
    clusters.join("")
}
