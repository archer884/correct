use num::traits::Float;
use strsim;

/// Gets the best jaro match for the input word.
pub fn best_match<'a>(dict: &'a [String], target: &'a str) -> Option<&'a str> {
    let (_, word) = dict.iter()
        .map(|word| (strsim::jaro(word, target), word))
        .fold((0.0, ""), |(ar, aw), (br, bw)| if br > ar { (br, bw) } else { (ar, aw) });

    if word != "" { Some(&word) } else { None }
}

/// Gets the word with the least levenshtein distance from the target word.
pub fn best_levenshtein_match<'a>(dict: &'a [String], target: &'a str) -> Option<&'a str> {
    let (_, word) = dict.iter()
        .map(|word| (strsim::levenshtein(word, target), word))
        .fold((usize::max_value(), ""), |(ar, aw), (br, bw)| if br < ar { (br, bw) } else { (ar, aw) });

    if word != "" { Some(&word) } else { None }
}


/// Gets the word with the least hamming distance from the target word.
pub fn best_hamming_match<'a>(dict: &'a [String], target: &'a str) -> Option<&'a str> {
    let (_, word) = dict.iter()
        .filter_map(|word| strsim::hamming(word, target).map(|score| (score, word)).ok())
        .fold((usize::max_value(), ""), |(ar, aw), (br, bw)| if br < ar { (br, bw) } else { (ar, aw) });

    if word != "" { Some(&word) } else { None }
}
