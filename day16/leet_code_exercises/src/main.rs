struct Solution {}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups: std::collections::HashMap<[u8; 26], Vec<String>> =
            std::collections::HashMap::new();
        for s in strs {
            let mut chars: [u8; 26] = [0; 26];
            for char in s.chars() {
                chars[char.to_ascii_lowercase() as usize - b'a' as usize] += 1;
            }
            groups
                .entry(chars)
                .and_modify(|v| v.push(s.clone()))
                .or_insert(vec![s]);
        }
        groups.values().cloned().collect()
    }

    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut freq: std::collections::HashMap<i32, i32> =
            std::collections::HashMap::with_capacity(nums.len());
        for n in nums {
            freq.entry(n).and_modify(|n| *n += 1).or_insert(1);
        }
        let mut count: Vec<_> = freq.into_iter().collect();
        count.sort_by(|a, b| b.1.cmp(&a.1));
        count.iter().take(k as usize).map(|x| x.0).collect()
    }
}

fn main() {
    let strs: Vec<String> = vec![
        "eat".to_string(),
        "tea".to_string(),
        "tan".to_string(),
        "ate".to_string(),
        "nat".to_string(),
        "bat".to_string(),
    ];
    println!(
        "https://leetcode.com/problems/group-anagrams/ case 1 answer: {:?}",
        Solution::group_anagrams(strs)
    );

    let nums = vec![1, 1, 1, 2, 2, 3];
    let k = 2;
    println!(
        "https://leetcode.com/problems/top-k-frequent-elements/ case 1 answer: {:?}",
        Solution::top_k_frequent(nums, k)
    );
}
