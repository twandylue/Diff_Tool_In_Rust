use std::cmp;

pub struct Differ {
    pub new_text: String,
    pub old_text: String,
}

impl Differ {
    fn compute(old_text: &Vec<String>, new_text: &Vec<String>) -> Vec<String> {
        let lcs = compute_lcs_matrix_dp(new_text, old_text);
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
        let new_text = &self
            .new_text
            .chars()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let old_text = &self
            .old_text
            .chars()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        return Self::compute(old_text, new_text);
    }

    pub fn diff_by_words(&self) -> Vec<String> {
        let new_text = &self
            .new_text
            .split(" ")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let old_text = &self
            .old_text
            .split(" ")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        return Self::compute(old_text, new_text);
    }
}

fn compute_lcs_string_dp(text1: &Vec<char>, text2: &Vec<char>) -> String {
    let mut result = String::new();
    let mut i = text1.len();
    let mut j = text2.len();

    let lcs = compute_lcs_matrix_dp(text1, text2);

    while i != 0 && j != 0 {
        if text1[i - 1] == text2[j - 1] {
            result.push(text1[i - 1]);
            i -= 1;
            j -= 1;
        } else if lcs[j - 1][i] <= lcs[j][i - 1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }

    return result.chars().rev().collect();
}

fn compute_lcs_matrix_dp<T>(new_text: &Vec<T>, old_text: &Vec<T>) -> Vec<Vec<i32>>
where
    T: PartialEq,
{
    let mut result = vec![vec![0; new_text.len() + 1]; old_text.len() + 1];
    for i in 0..old_text.len() + 1 {
        for j in 0..new_text.len() + 1 {
            if i == 0 || j == 0 {
                result[i][j] = 0;
            } else if old_text[i - 1] == new_text[j - 1] {
                result[i][j] = result[i - 1][j - 1] + 1
            } else {
                result[i][j] = cmp::max(result[i - 1][j], result[i][j - 1])
            }
        }
    }

    return result;
}

fn compute_lcs_len_recursive(i: usize, j: usize, text1: &Vec<char>, text2: &Vec<char>) -> u32 {
    if i == 0 || j == 0 {
        return 0;
    }

    if text1[i - 1] == text2[j - 1] {
        return 1 + compute_lcs_len_recursive(i - 1, j - 1, text1, text2);
    }

    return cmp::max(
        compute_lcs_len_recursive(i - 1, j, text1, text2),
        compute_lcs_len_recursive(i, j - 1, text1, text2),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_lcs_len_recursive_test() {
        let s1: Vec<char> = String::from("abcd").chars().collect();
        let s2: Vec<char> = String::from("abcd").chars().collect();
        let actual = compute_lcs_len_recursive(s1.len(), s2.len(), &s1, &s2);
        assert_eq!(actual, 4);
    }

    #[test]
    fn compute_lcs_matrix_dp_test() {
        let s1: Vec<char> = String::from("bdcaba").chars().collect();
        let s2: Vec<char> = String::from("abcbdab").chars().collect();
        let actual = compute_lcs_matrix_dp(&s1, &s2);

        assert_eq!(actual[0][0], 0);
        assert_eq!(actual.len(), s2.len() + 1);
        assert_eq!(actual[0].len(), s1.len() + 1);
    }

    #[test]
    fn compute_lcs_string_dp_test() {
        let s1: Vec<char> = String::from("abcddddd").chars().collect();
        let s2: Vec<char> = String::from("abzdcd").chars().collect();
        let actual = compute_lcs_string_dp(&s1, &s2);

        assert_eq!(actual, "abcd");
    }

    #[test]
    fn diff_test_chars_1() {
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
    fn diff_test_chars_2() {
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
    fn diff_test_chars_3() {
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
    fn diff_test_chars_4() {
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
    fn diff_test_words_1() {
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
    fn diff_test_words_2() {
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
    fn diff_test_words_3() {
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
    fn diff_test_words_4() {
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
    fn diff_test_words_5() {
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