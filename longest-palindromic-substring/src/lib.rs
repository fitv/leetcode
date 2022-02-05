pub struct Solution {}

#[allow(unused_comparisons)]
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut max = 0;
        let mut range = (0, 0);
        let chars: Vec<_> = s.chars().collect();

        let mut find_longest = |mut low, mut high| {
            while low >= 0 && high < chars.len() && chars[low] == chars[high] {
                if high - low + 1 > max {
                    max = high - low + 1;
                    range = (low, high);
                }

                if low == 0 {
                    break;
                }
                low -= 1;
                high += 1;
            }
        };

        for i in 0..s.len() {
            find_longest(i, i);
            find_longest(i, i + 1);
        }
        chars.get(range.0..=range.1).unwrap().iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_longest_palindrome() {
        let s = String::from("cbbd");
        assert_eq!(Solution::longest_palindrome(s), "bb");

        let s = String::from("ac");
        assert_eq!(Solution::longest_palindrome(s), "a");

        let s = String::from("ccc");
        assert_eq!(Solution::longest_palindrome(s), "ccc");

        let s = String::from("aaabaaaa");
        assert_eq!(Solution::longest_palindrome(s), "aaabaaa");

        let s = String::from("xaabacxcabaaxcabaax");
        assert_eq!(Solution::longest_palindrome(s), "xaabacxcabaax");

        let s = String::from("civilwartestingwhetherthatnaptionoranynartionsoconceivedandsodedicatedcanlongendureWeareqmetonagreatbattlefiemldoftzhatwarWehavecometodedicpateaportionofthatfieldasafinalrestingplaceforthosewhoheregavetheirlivesthatthatnationmightliveItisaltogetherfangandproperthatweshoulddothisButinalargersensewecannotdedicatewecannotconsecratewecannothallowthisgroundThebravelmenlivinganddeadwhostruggledherehaveconsecrateditfaraboveourpoorponwertoaddordetractTgheworldadswfilllittlenotlenorlongrememberwhatwesayherebutitcanneverforgetwhattheydidhereItisforusthelivingrathertobededicatedheretotheulnfinishedworkwhichtheywhofoughtherehavethusfarsonoblyadvancedItisratherforustobeherededicatedtothegreattdafskremainingbeforeusthatfromthesehonoreddeadwetakeincreaseddevotiontothatcauseforwhichtheygavethelastpfullmeasureofdevotionthatweherehighlyresolvethatthesedeadshallnothavediedinvainthatthisnationunsderGodshallhaveanewbirthoffreedomandthatgovernmentofthepeoplebythepeopleforthepeopleshallnotperishfromtheearth");
        assert_eq!(Solution::longest_palindrome(s), "ranynar");
    }
}
