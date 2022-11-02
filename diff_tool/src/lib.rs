use ::std::cmp;

pub struct Diff {
    addition: Vec<char>,
    removal: Vec<char>,
    unchanged: Vec<char>,
}

impl Diff {
    pub fn new() -> Self {
        Diff {
            addition: Vec::new(),
            removal: Vec::new(),
            unchanged: Vec::new(),
        }
    }
}

pub fn diff(text1: &Vec<char>, text2: &Vec<char>) -> Diff {
    let lcs = compute_lcs_len_dp(text1, text2);
    let mut i = text1.len();
    let mut j = text2.len();

    let mut result = Diff::new();

    while i != 0 || j != 0 {
        if i == 0 {
            result.addition.push(text2[j - 1]);
            j -= 1;
        } else if j == 0 {
            result.removal.push(text1[i - 1]);
            i -= 1;
        } else if text1[i - 1] == text2[j - 1] {
            result.unchanged.push(text1[i - 1]);
            i -= 1;
            j -= 1;
        } else if lcs[j - 1][i] <= lcs[j][i - 1] {
            result.addition.push(text1[i - 1]);
            i -= 1;
        } else {
            result.removal.push(text2[j - 1]);
            j -= 1;
        }
    }

    result.addition.reverse();
    result.removal.reverse();
    result.unchanged.reverse();
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

pub fn compute_lcs_len_dp(text1: &Vec<char>, text2: &Vec<char>) -> Vec<Vec<i32>> {
    let mut result = vec![vec![0; text1.len() + 1]; text2.len() + 1];
    for i in 0..text2.len() + 1 {
        for j in 0..text1.len() + 1 {
            if i == 0 || j == 0 {
                result[i][j] = 0;
            } else if text2[i - 1] == text1[j - 1] {
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
    fn diff_test_chars() {
        let new: Vec<char> = String::from("abcdefghi").chars().collect();
        let old: Vec<char> = String::from("azedbcz").chars().collect();
        let r = diff(&new, &old);

        assert_eq!(r.addition.iter().collect::<String>(), "defghi");
        assert_eq!(r.unchanged.iter().collect::<String>(), "abc");
        assert_eq!(r.removal.iter().collect::<String>(), "zedz");
    }

    #[test]
    fn diff_test_words() {
        let new: Vec<char> = String::from("This is Andy").chars().collect();
        let old: Vec<char> = String::from("This is Amy").chars().collect();
        let r = diff(&new, &old);

        assert_eq!(r.addition.iter().collect::<String>(), "nd");
        assert_eq!(r.removal.iter().collect::<String>(), "m");
    }
}
