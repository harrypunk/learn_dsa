pub mod dynamic {
    pub fn longest_pal(s: String) -> String {
        s
    }

    /**
     * expand and calc palindrome arm length
     * skip element within (left, right)
     */
    pub fn expand(s: &str, mut left: usize, mut right: usize) -> usize {
        let bytes = s.as_bytes();
        while right < s.len() && bytes[left] == bytes[right] {
            left -= 1;
            right += 1;
        }
        (right - left - 2) / 2
    }

    pub fn add_holder(s: &str) -> String {
        let mut res = String::from('#');

        for i in s.chars() {
            res.push(i);
            res.push('#')
        }
        res
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn dynamic_pal() {
            let s1 = "acebabece";
            let sub1 = expand(s1, 3, 5);
            assert_eq!(3, sub1);
            let sub2 = expand(s1, 4, 4);
            assert_eq!(3, sub2)
        }

        #[test]
        fn dynamic_holder() {
            let s1 = "abcd";
            let s2 = add_holder(s1);
            assert_eq!("#a#b#c#d#", s2)
        }
    }
}
