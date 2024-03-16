struct Solution {}

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let uniques: std::collections::HashSet<i32> = nums.iter().cloned().collect();
        uniques.len() != nums.len()
    }

    pub fn is_anagram(s: String, t: String) -> bool {
        let mut s_freq: std::collections::HashMap<char, i32> = std::collections::HashMap::new();
        let mut t_freq: std::collections::HashMap<char, i32> = std::collections::HashMap::new();
        for char in s.chars() {
            s_freq.entry(char).and_modify(|f| *f += 1).or_insert(1);
        }
        for char in t.chars() {
            t_freq.entry(char).and_modify(|f| *f += 1).or_insert(1);
        }
        s_freq == t_freq
    }

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut visited: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
        for (idx, num) in nums.into_iter().enumerate() {
            let search = target - num;
            match visited.get(&search) {
                Some(&search_idx) => return vec![search_idx as i32, idx as i32],
                None => visited.insert(num, idx as i32),
            };
        }
        unreachable!();
    }
}

fn main() {
    let nums = vec![1, 2, 3, 1];
    println!(
        "https://leetcode.com/problems/contains-duplicate/ case 1 answer: {}",
        Solution::contains_duplicate(nums)
    );

    let s = String::from("anagram");
    let t = String::from("nagaram");
    println!(
        "https://leetcode.com/problems/valid-anagram/ case 1 answer: {}",
        Solution::is_anagram(s, t)
    );

    let nums = vec![2, 7, 11, 15];
    let target = 9;
    println!(
        "https://leetcode.com/problems/two-sum/ case 1 answer: {:?}",
        Solution::two_sum(nums, target)
    );
}
