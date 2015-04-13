use col::CappedList;
use num::traits::Float;
use std::cmp::Ordering;
use strsim;

/// Gets the best jaro match for the input word.
pub fn best_match<'a>(dict: &'a [String], target: &'a str) -> Option<&'a str> {
    let (_, word) = dict.iter()
        .map(|word| (strsim::jaro(word, target), word))
        .fold((0.0, ""), |(ar, aw), (br, bw)| if br > ar { (br, bw) } else { (ar, aw) });

    if word != "" { Some(&word) } else { None }
}

/// Gets the top five best jaro matches for the input word.
pub fn best_matches<'a>(dict: &'a [String], target: &'a str, count: usize) -> Vec<(f64, String)> {
    let mut items = CappedList::<(f64, &str)>::new(count);
    for word in dict {
        let score = strsim::jaro(word, target);
        if items.list.back().map(|t| t.0).unwrap_or(0.0) < score { items.push_max((score, word)); }
    }

    let mut vec: Vec<_> = items.list.iter().map(|pair| (pair.0, pair.1.to_string())).collect();
    vec.sort_by(|a,b| b.0.partial_cmp(&a.0).unwrap_or(Ordering::Equal));
    vec
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
