pub struct Solution {}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> Vec<usize> {
        if haystack.is_empty() || needle.is_empty() {
            return vec![];
        }
        let content = haystack.into_bytes();
        let pat = needle.into_bytes();
        let next_tab = Self::get_next(&pat);
        let mut res = vec![];
        let mut j = 0;
        for (i, &c) in content.iter().enumerate() {
            while j > 0 && c != pat[j] {
                j = next_tab[j - 1];
            }
            if c == pat[j] {
                j += 1;
            }
            if j == pat.len() {
                res.push(i + 1 - j);
                j = next_tab[j - 1];
            }
        }
        res
    }
    // build a next table
    fn get_next(pattern: &Vec<u8>) -> Vec<usize> {
        let mut tab = vec![0];

        for i in 1..pattern.len() {
            let mut j = tab[i - 1];
            while j > 0 && pattern[j] != pattern[i] {
                j = tab[j - 1];
            }

            tab.push(if pattern[j] == pattern[i] { j + 1 } else { j });
        }
        tab
    }
}
