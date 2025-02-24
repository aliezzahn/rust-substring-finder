use rust_substring_finder::Solution;

fn main() {
    let s = "barfoothefoobarman".to_string();
    let words = vec!["foo".to_string(), "bar".to_string()];
    
    // Call the find_substring method
    let result = Solution::find_substring(s, words);
    
    // Print the result
    println!("Result: {:?}", result);
}