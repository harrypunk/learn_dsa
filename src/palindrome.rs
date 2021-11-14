pub mod manacher {
    pub fn longest_pal(s1: String) -> String {
        if s1.len() == 0 || s1.len() == 1 {
            return s1;
        }

        let mut start = 0;
        let mut end = 0;

        let mut arm_len: Vec<usize> = Vec::new();

        let mut right = 0usize;
        // center of lastest max palindrome
        let mut j = 0usize;

        let s = add_holder(&s1);
        for i in 0..s.len() {
            //log::debug!("i {}, j {}, right {}", i, j, right);
            let current_arm_len: usize;
            // get min radius from symmetric i_sym
            if right >= i && j > i - j {
                let i_sym = j * 2 - i;
                let min_arm_len = std::cmp::min(arm_len[i_sym], right - i);
                current_arm_len = expand(&s, i - min_arm_len, i + min_arm_len);
            } else {
                current_arm_len = expand(&s, i, i);
            }

            //{
                //log::debug!("i {}, current_arm_len {}", i, current_arm_len);
            //}

            arm_len.push(current_arm_len);
            if i + current_arm_len > right {
                j = i;
                right = i + current_arm_len;
            }

            if current_arm_len * 2 + 1 > end - start {
                start = i - current_arm_len;
                end = i + current_arm_len;
            }
        }

        let augmented = s[start..=end].to_string();
        remove_holder(&augmented)
    }

    /**
     * expand and calc palindrome arm length
     * skip element within (left, right)
     */
    pub fn expand(s: &str, l: usize, r: usize) -> usize {
        let mut left = l;
        let mut right = r;
        let bytes = s.as_bytes();
        while right < s.len() {
            if bytes[left] == bytes[right] {
                if left == 0 || right == s.len() - 1 {
                    break;
                } else {
                    left -= 1;
                    right += 1;
                }
            } else {
                left += 1;
                right -= 1;
                break;
            }
        }
        /*
         * expand exits
         * 1. char_left != char_right
         * 2. char0 == char_len, s is palindome
         */
        (right - left) / 2
    }

    pub fn add_holder(s: &str) -> String {
        let mut res = String::from('#');

        for i in s.chars() {
            res.push(i);
            res.push('#')
        }
        res
    }

    pub fn remove_holder(s: &str) -> String {
        let mut res = String::new();

        for i in s.chars() {
            if i != '#' {
                res.push(i);
            }
        }
        res
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        //use crate::util::init_log;

        #[test]
        fn mana_expand() {
            let s1 = "acebabece";
            let sub1 = expand(s1, 3, 5);
            assert_eq!(3, sub1);
            let sub2 = expand(s1, 4, 4);
            assert_eq!(3, sub2)
        }

        #[test]
        fn mana_holder() {
            let s1 = "abcd";
            let s2 = add_holder(s1);
            assert_eq!("#a#b#c#d#", s2);
            let s3 = remove_holder(&s2);
            assert_eq!(s3, s1);
        }

        #[test]
        fn mana_pal() {
            let s1 = "acebabece";
            let res = longest_pal(s1.to_string());
            assert_eq!(res, "cebabec");

            let s2 = "acabddbecabccbae";
            let res2 = longest_pal(s2.to_string());
            assert_eq!(res2, "abccba");
        }

        #[test]
        fn mana_len2() {
            let leet162 = "bb";
            let res162 = longest_pal(leet162.to_string());
            assert_eq!("bb", res162);
            let exp = add_holder(leet162);
            assert_eq!("#b#b#", exp);
            let right = expand(&exp, 2, 2);
            assert_eq!(2, right);
        }

        #[test]
        fn mana_159() {
            //init_log();
            let s = "cbbd";
            let res = longest_pal(s.to_string());
            assert_eq!("bb", res);
        }
    }
}
