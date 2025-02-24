use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        if s.is_empty() || words.is_empty() {
            return vec![];
        }

        let word_len = words[0].len();
        let total_len = word_len * words.len();
        if s.len() < total_len {
            return vec![];
        }

        let mut result = Vec::with_capacity(s.len() / word_len);
        let s_chars = s.as_bytes();

        let mut word_count = HashMap::with_capacity(words.len());
        for word in &words {
            *word_count.entry(word.as_str()).or_insert(0) += 1;
        }

        for offset in 0..word_len {
            let mut seen = HashMap::with_capacity(words.len());
            let mut count = 0;
            let mut left = offset;

            for i in (offset..=s.len() - word_len).step_by(word_len) {
                let word = unsafe { std::str::from_utf8_unchecked(&s_chars[i..i + word_len]) };

                if let Some(&expected) = word_count.get(word) {
                    let seen_entry = seen.entry(word).or_insert(0);
                    *seen_entry += 1;
                    count += 1;

                    if *seen_entry > expected {
                        loop {
                            let left_word = unsafe { std::str::from_utf8_unchecked(&s_chars[left..left + word_len]) };
                            let left_entry = seen.get_mut(left_word).unwrap();
                            *left_entry -= 1;
                            count -= 1;
                            left += word_len;

                            if left_word == word {
                                break;
                            }
                        }
                    }

                    if count == words.len() {
                        result.push(left as i32);
                    }
                } else {
                    seen.clear();
                    count = 0;
                    left = i + word_len;
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    
    #[test]
    fn test_find_substring() {
        let s = "barfoothefoobarman".to_string();
        let words = vec!["foo".to_string(), "bar".to_string()];
        let mut result = Solution::find_substring(s, words);
        result.sort();
        assert_eq!(result, vec![0, 9]);
    }
    
    #[test]
    fn test_empty_string() {
        let s = "".to_string();
        let words = vec!["word".to_string()];
        assert_eq!(Solution::find_substring(s, words), vec![]);
    }
    
    #[test]
    fn test_no_match() {
        let s = "abcdef".to_string();
        let words = vec!["gh".to_string(), "ij".to_string()];
        assert_eq!(Solution::find_substring(s, words), vec![]);
    }
}