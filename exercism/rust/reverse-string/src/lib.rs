// https://craftofcoding.wordpress.com/tag/grapheme-cluster/
// https://crates.io/crates/unicode-segmentation
// https://stackoverflow.com/questions/27996430/reversing-a-string-in-rust
use unicode_segmentation::UnicodeSegmentation;
pub fn reverse(input: &str) -> String {
    //unimplemented!("Write a function to reverse {}", input);
    let result:String =
        input
        .graphemes(true)
        .rev()
        .collect::<String>();
    result
}
