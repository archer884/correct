use strsim;

pub fn best_match<'a>(dict: &'a [String], target: &'a str) -> Option<&'a str> {
    let (_, word) = dict.iter()
        .filter_map(|word| strsim::hamming(word, target).map(|score| (score, word)).ok())
        .fold((0, ""), |(ar, aw), (br, bw)| if br > ar { (br, bw) } else { (ar, aw) });

    if word != "" { Some(&word) } else { None }
}
