use std::collections::{HashMap, HashSet};
use regex::Regex;
use ndarray::Array1;

fn simty(s1: &str, s2: &str) -> f64 {
    let binding1 = s1.to_lowercase();
    let words1: Vec<&str> = binding1.split_whitespace().collect();
    let binding2 = s2.to_lowercase();
    let words2: Vec<&str> = binding2.split_whitespace().collect();
    let unique_words: Vec<String> = words1.iter().chain(words2.iter())
        .map(|&w| w.to_string())
        .collect::<std::collections::HashSet<String>>()
        .into_iter()
        .collect();
    let freq1 = word_frequency(&words1);
    let freq2 = word_frequency(&words2);
    let vec1 = create_vector(&unique_words, &freq1);
    let vec2 = create_vector(&unique_words, &freq2);
    let dot_product = vec1.dot(&vec2);
    let norm1 = vec1.dot(&vec1).sqrt();
    let norm2 = vec2.dot(&vec2).sqrt();
    dot_product / (norm1 * norm2)
}

fn word_frequency(words: &[&str]) -> HashMap<String, usize> {
    let mut freq_map = HashMap::new();
    for word in words {
        *freq_map.entry(word.to_string()).or_insert(0) += 1;
    }
    freq_map
}

fn create_vector(unique_words: &[String], freq_map: &HashMap<String, usize>) -> Array1<f64> {
    Array1::from_vec(
        unique_words
            .iter()
            .map(|word| *freq_map.get(word).unwrap_or(&0) as f64)
            .collect()
    )
}

fn detect_septor_pattern(text: &str) -> Option<String> {
    let char_freq = count_non_alphanumeric_chars(text);
    let potential_separators = identify_frequent_chars(&char_freq);

    let regex_pattern = create_regex_pattern(&potential_separators);
    let potential_words_and_seps = split_with_regex(text, &regex_pattern);

    let separator_patterns = extract_separator_patterns(&potential_words_and_seps);
    let best_pattern = identify_best_pattern(&separator_patterns)?;

    if validate_pattern(&best_pattern, text) {
        Some(best_pattern)
    } else {
        Some(best_pattern)
    }
}

fn count_non_alphanumeric_chars(text: &str) -> HashMap<char, usize> {
    text.chars()
        .filter(|c| !c.is_alphanumeric())
        .fold(HashMap::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        })
}

fn identify_frequent_chars(char_freq: &HashMap<char, usize>) -> HashSet<char> {
    let total_chars: usize = char_freq.values().sum();
    let threshold = (total_chars as f64 * 0.05).max(1.0) as usize; // Ensure at least 1
    char_freq
        .iter()
        .filter(|(_, &count)| count >= threshold)
        .map(|(&c, _)| c)
        .collect()
}

fn create_regex_pattern(separators: &HashSet<char>) -> Regex {
    if separators.is_empty() {
        return Regex::new(r"\s+").unwrap_or_else(|_| Regex::new(r"").unwrap());
    }

    let sep_pattern = regex::escape(&separators.iter().collect::<String>());
    let pattern = format!(
        r"(?x)
        (?:
            [a-zA-Z0-9]+
            (?:[{0}]+[a-zA-Z0-9]+)+
        )
        |
        (?:
            \S+
        )",
        sep_pattern
    );

    Regex::new(&pattern).unwrap_or_else(|e| {
        eprintln!("Failed to create regex pattern: {}", e);
        Regex::new(r"\S+").unwrap()
    })
}

fn split_with_regex<'a>(text: &'a str, regex: &Regex) -> Vec<&'a str> {
    regex.find_iter(text).map(|m| m.as_str()).collect()
}

fn extract_separator_patterns(words_and_seps: &[&str]) -> Vec<String> {
    words_and_seps
        .iter()
        .filter(|&&s| !s.chars().any(char::is_alphanumeric))
        .map(|&s| s.to_string())
        .collect()
}

fn identify_best_pattern(patterns: &[String]) -> Option<String> {
    patterns
        .iter()
        .max_by_key(|p| patterns.iter().filter(|&x| x == *p).count())
        .cloned()
}


fn removesep(text: &str, separator_pattern: &str) -> String {
    if separator_pattern.is_empty() {
        return text.to_string();
    }

    let sep_pattern = regex::escape(separator_pattern);
    let regex = Regex::new(&format!(
        r"[{}]+",
        sep_pattern
    )).unwrap_or_else(|_| Regex::new(r"").unwrap());

    regex.replace_all(text, "").into_owned()
}

fn validate_pattern(pattern: &str, text: &str) -> bool {
    let words = extract_words(text, pattern);
    words.len() > 1 && words.iter().all(|w| !w.is_empty())
}

fn extract_words(text: &str, separator_pattern: &str) -> Vec<String> {
    if separator_pattern.is_empty() {
        return text.split_whitespace().map(String::from).collect();
    }

    let sep_pattern = regex::escape(separator_pattern);
    let regex = Regex::new(&format!(
        r"(?:[{0}]|[{0}]+\s+[{0}]+|\s+[{0}]+|[{0}]+\s+|\s+)",
        sep_pattern
    )).unwrap();

    regex
        .split(text)
        .map(|s| s.replace(&sep_pattern, ""))
        .filter(|s| !s.is_empty())
        .collect()
}

fn compseps(first: &str, second: &str) -> String {
    fn remove_separators(s: &str) -> String {
        s.chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace())
            .collect()
    }

    let first_clean = remove_separators(first);
    let second_clean = remove_separators(second);

    let first_lower = first_clean.to_lowercase();
    let second_lower = second_clean.to_lowercase();

    if second_lower.contains(&first_lower) {
        first.to_string()
    } else {
        second.to_string()
    }
}

fn comors(first: &str, second: &str) -> String {
    fn remove_separators(s: &str) -> String {
        s.chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace())
            .collect()
    }

    fn is_space_delimited(s: &str) -> bool {
        let words: Vec<&str> = s.split_whitespace().collect();
        words.iter().all(|word| word.len() == 1)
    }

    let first_clean = remove_separators(first);
    let mut second_clean = remove_separators(second);

    if is_space_delimited(&second_clean) {
        second_clean = second_clean.replace(" ", "");
    }

    let first_lower = first_clean.to_lowercase();
    let second_lower = second_clean.to_lowercase();

    if second_lower.contains(&first_lower) {
        first.to_string()
    } else {
        second.to_string()
    }
}

pub fn checkit(text1: &str, text2: &str) -> String {
    let use1 = text1.to_owned() + " .";
    let use2 = text2.to_owned() + " .";
    if let Some(pattern2) = detect_septor_pattern(&use2) {
        let cleaned_text2 = removesep(&use2, &pattern2);
        let result2 = compseps(&use1, &cleaned_text2);
        let result3 = comors(&use1, &cleaned_text2);
        let cosign1 = simty(&use1, &result2);
        let cosign2 = simty(&use1, &result3);
        log::debug!("separator removed cosign: {:?}, calc string: {:?}", &cosign1, &result2);
        if cosign1 < 0.5 {
            "pass".to_string()
        } else if cosign2 < 0.5 {
            "pass".to_string()
        } else {
            "match".to_string()
        }

    } else {
        let result4 = compseps(&use1, text2);
        let cosign3 = simty(&use1, &result4);
        log::debug!("as is cosign {:?}, calc string: {:?}", &cosign3, &result4);
        if cosign3 < 0.5 {
            "pass".to_string()
        } else {
            "match".to_string()
        }
    }
}
