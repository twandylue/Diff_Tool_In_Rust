use std::cmp;

pub struct Differ {
    pub new_text: String,
    pub old_text: String,
}

impl Differ {
    pub fn new(new_text: String, old_text: String) -> Self {
        Differ { new_text, old_text }
    }

    fn compute_lcs_matrix_dp<T>(new_text: &Vec<T>, old_text: &Vec<T>) -> Vec<Vec<i32>>
    where
        T: PartialEq,
    {
        let mut dp = vec![vec![0; new_text.len() + 1]; old_text.len() + 1];
        for i in 0..old_text.len() + 1 {
            for j in 0..new_text.len() + 1 {
                if i == 0 || j == 0 {
                    dp[i][j] = 0;
                } else if old_text[i - 1] == new_text[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1
                } else {
                    dp[i][j] = cmp::max(dp[i - 1][j], dp[i][j - 1])
                }
            }
        }

        return dp;
    }

    fn compute_lcs(old_text: &Vec<String>, new_text: &Vec<String>) -> Vec<String> {
        let lcs = Self::compute_lcs_matrix_dp(new_text, old_text);
        let mut i = new_text.len();
        let mut j = old_text.len();

        let mut result: Vec<String> = Vec::new();

        while i != 0 || j != 0 {
            if i == 0 {
                result.push(format!("-{}", old_text[j - 1]));
                j -= 1;
            } else if j == 0 {
                result.push(format!("+{}", new_text[i - 1]));
                i -= 1;
            } else if new_text[i - 1] == old_text[j - 1] {
                result.push(format!("{}", new_text[i - 1]));
                i -= 1;
                j -= 1;
            } else if lcs[j - 1][i] <= lcs[j][i - 1] {
                result.push(format!("+{}", new_text[i - 1]));
                i -= 1;
            } else {
                result.push(format!("-{}", old_text[j - 1]));
                j -= 1;
            }
        }

        result.reverse();
        return result;
    }

    pub fn diff_by_chars(&self) -> Vec<String> {
        let new_text = self
            .new_text
            .chars()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let old_text = self
            .old_text
            .chars()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        return Self::compute_lcs(&old_text, &new_text);
    }

    pub fn diff_by_words(&self) -> Vec<String> {
        let new_text = self
            .new_text
            .split_ascii_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        let old_text = self
            .old_text
            .split_ascii_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        return Self::compute_lcs(&old_text, &new_text);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diff_test_chars_ok_1() {
        let new = String::from("abcd");
        let old = String::from("abc");
        let content = Differ {
            new_text: new,
            old_text: old,
        };
        let actual = content.diff_by_chars();

        let expected = vec!["a", "b", "c", "+d"];
        for i in 0..actual.len() {
            assert_eq!(actual[i], expected[i]);
        }
    }

    #[test]
    fn diff_test_chars_ok_2() {
        let new = String::from("abcd");
        let old = String::from("");
        let content = Differ {
            new_text: new,
            old_text: old,
        };
        let actual = content.diff_by_chars();

        let expected = vec!["+a", "+b", "+c", "+d"];
        for i in 0..actual.len() {
            assert_eq!(actual[i], expected[i]);
        }
    }

    #[test]
    fn diff_test_chars_ok_3() {
        let new = String::from("");
        let old = String::from("abcd");
        let content = Differ {
            new_text: new,
            old_text: old,
        };
        let actual = content.diff_by_chars();

        let expected = vec!["-a", "-b", "-c", "-d"];
        for i in 0..actual.len() {
            assert_eq!(actual[i], expected[i]);
        }
    }

    #[test]
    fn diff_test_chars_ok_4() {
        let new = String::from("abecd");
        let old = String::from("zaabck");
        let content = Differ {
            new_text: new,
            old_text: old,
        };
        let actual = content.diff_by_chars();

        let expected = vec!["-z", "-a", "a", "b", "+e", "c", "-k", "+d"];
        for i in 0..actual.len() {
            assert_eq!(actual[i], expected[i]);
        }
    }

    #[test]
    fn diff_test_words_ok_1() {
        let new = String::from("He is Andy");
        let old = String::from("She is Amy");
        let content = Differ {
            new_text: new,
            old_text: old,
        };

        let actual = content.diff_by_words();
        let expected = vec!["-She", "+He", "is", "-Amy", "+Andy"];
        for i in 0..actual.len() {
            assert_eq!(actual[i], expected[i]);
        }
    }

    #[test]
    fn diff_test_words_ok_2() {
        let new = String::from("He is Andy");
        let old = String::from("He is");
        let content = Differ {
            new_text: new,
            old_text: old,
        };

        let actual = content.diff_by_words();
        let expected = vec!["He", "is", "+Andy"];
        for i in 0..actual.len() {
            assert_eq!(actual[i], expected[i]);
        }
    }

    #[test]
    fn diff_test_words_ok_3() {
        let new = String::from("is Andy");
        let old = String::from("He is Andy");
        let content = Differ {
            new_text: new,
            old_text: old,
        };

        let actual = content.diff_by_words();
        let expected = vec!["-He", "is", "Andy"];
        for i in 0..actual.len() {
            assert_eq!(actual[i], expected[i]);
        }
    }

    #[test]
    fn diff_test_words_ok_4() {
        let new = String::from("He is Andy She is Anne");
        let old = String::from("He is Andy I am Amy");
        let content = Differ {
            new_text: new,
            old_text: old,
        };

        let actual = content.diff_by_words();
        println!("actual: {:?}", actual);
        let expected = vec![
            "He", "is", "Andy", "-I", "-am", "-Amy", "+She", "+is", "+Anne",
        ];
        for i in 0..actual.len() {
            assert_eq!(actual[i], expected[i]);
        }
    }

    #[test]
    fn diff_test_words_ok_5() {
        let new = String::from("He is Andy I am Amyyy She is Anne");
        let old = String::from("He is Andy I am Amy");
        let content = Differ {
            new_text: new,
            old_text: old,
        };

        let actual = content.diff_by_words();
        let expected = vec![
            "He", "is", "Andy", "I", "am", "-Amy", "+Amyyy", "+She", "+is", "+Anne",
        ];
        for i in 0..actual.len() {
            assert_eq!(actual[i], expected[i]);
        }
    }
}
