use ::std::cmp;

// TODO: still working...
// pub struct DiffWords {
//     addition: Vec<String>,
//     removal: Vec<String>,
//     unchanged: Vec<String>,
// }
//
// impl DiffWords {
//     pub fn new() -> Self {
//         DiffWords {
//             addition: Vec::new(),
//             removal: Vec::new(),
//             unchanged: Vec::new(),
//         }
//     }
// }

// pub fn diff_words(text1: &Vec<&str>, text2: &Vec<char>) -> DiffWords {}

pub fn diff_chars(new_text: &Vec<char>, old_text: &Vec<char>) -> Vec<String> {
    let lcs = compute_lcs_len_dp(new_text, old_text);
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

pub fn compute_lcs_string_dp(text1: &Vec<char>, text2: &Vec<char>) -> String {
    let mut result = String::new();
    let mut i = text1.len();
    let mut j = text2.len();

    let lcs = compute_lcs_len_dp(text1, text2);

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

pub fn compute_lcs_len_dp(new_text: &Vec<char>, old_text: &Vec<char>) -> Vec<Vec<i32>> {
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

pub fn compute_lcs_len_recursive(i: usize, j: usize, text1: &Vec<char>, text2: &Vec<char>) -> u32 {
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
    fn compute_lcs_len_compute_lcs_len_recursive_test() {
        let s1: Vec<char> = String::from("abcd").chars().collect();
        let s2: Vec<char> = String::from("abcd").chars().collect();
        let r = compute_lcs_len_recursive(s1.len(), s2.len(), &s1, &s2);
        assert_eq!(r, 4);
    }

    #[test]
    fn compute_lcs_len_dp_test() {
        let s1: Vec<char> = String::from("bdcaba").chars().collect();
        let s2: Vec<char> = String::from("abcbdab").chars().collect();
        let r = compute_lcs_len_dp(&s1, &s2);

        assert_eq!(r[0][0], 0);
        assert_eq!(r.len(), s2.len() + 1);
        assert_eq!(r[0].len(), s1.len() + 1);
    }

    #[test]
    fn compute_lcs_string_dp_test() {
        let s1: Vec<char> = String::from("abcddddd").chars().collect();
        let s2: Vec<char> = String::from("abzdcd").chars().collect();
        let r = compute_lcs_string_dp(&s1, &s2);

        assert_eq!(r, "abcd");
    }

    #[test]
    fn diff_test_chars_1() {
        let new: Vec<char> = String::from("abcd").chars().collect();
        let old: Vec<char> = String::from("abc").chars().collect();
        let r = diff_chars(&new, &old);

        let expected = vec!["a", "b", "c", "+d"];
        for i in 0..r.len() {
            assert_eq!(r[i], expected[i]);
        }
    }

    #[test]
    fn diff_test_chars_2() {
        let new: Vec<char> = String::from("abcd").chars().collect();
        let old: Vec<char> = String::from("").chars().collect();
        let r = diff_chars(&new, &old);

        let expected = vec!["+a", "+b", "+c", "+d"];
        for i in 0..r.len() {
            assert_eq!(r[i], expected[i]);
        }
    }

    #[test]
    fn diff_test_chars_3() {
        let new: Vec<char> = String::from("").chars().collect();
        let old: Vec<char> = String::from("abcd").chars().collect();
        let r = diff_chars(&new, &old);

        let expected = vec!["-a", "-b", "-c", "-d"];
        for i in 0..r.len() {
            assert_eq!(r[i], expected[i]);
        }
    }

    #[test]
    fn diff_test_chars_4() {
        let new: Vec<char> = String::from("abecd").chars().collect();
        let old: Vec<char> = String::from("zaabck").chars().collect();
        let r = diff_chars(&new, &old);

        let expected = vec!["-z", "-a", "a", "b", "+e", "c", "-k", "+d"];
        for i in 0..r.len() {
            assert_eq!(r[i], expected[i]);
        }
    }

    // #[test]
    // fn diff_test_words() {
    //     let new: Vec<char> = String::from("This is Andy").chars().collect();
    //     let old: Vec<char> = String::from("This is Amy").chars().collect();
    //     let r = diff_chars(&new, &old);
    //
    //     // assert_eq!(r.addition.iter().collect::<String>(), "nd");
    //     // assert_eq!(r.removal.iter().collect::<String>(), "m");
    // }
}
