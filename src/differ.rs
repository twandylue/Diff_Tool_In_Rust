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

    fn compute_lev(text1: String, text2: String) -> Vec<(char, usize, char)> {
        let c1 = text1.chars().collect::<Vec<char>>();
        let c2 = text2.chars().collect::<Vec<char>>();
        let n = text1.len();
        let m = text2.len();
        let mut tables = vec![vec![0; m + 1]; n + 1];
        tables[0][0] = 0;
        let mut actions = vec![vec!['-'; m + 1]; n + 1];
        actions[0][0] = 'I';

        for i in 1..n + 1 {
            actions[i][0] = 'R';
            tables[i][0] = i;
        }

        for j in 1..m + 1 {
            actions[0][j] = 'A';
            tables[0][j] = j;
        }

        for i in 1..n + 1 {
            for j in 1..m + 1 {
                if c1[i - 1] == c2[j - 1] {
                    actions[i][j] = 'I';
                    tables[i][j] = tables[i - 1][j - 1];
                    continue;
                }

                let remove = tables[i - 1][j];
                let add = tables[i][j - 1];

                tables[i][j] = remove;
                actions[i][j] = 'R';

                if add < tables[i][j] {
                    tables[i][j] = add;
                    actions[i][j] = 'A';
                }

                tables[i][j] += 1;
            }
        }

        // trace_cache(&tables, &actions, &text1, &text2);

        let mut patch: Vec<(char, usize, char)> = Vec::new();
        let mut n1 = n;
        let mut n2 = m;
        while n1 > 0 || n2 > 0 {
            let action = &actions[n1][n2];
            if *action == 'A' {
                n2 -= 1;
                patch.push(('A', n2, c2[n2]));
            } else if *action == 'R' {
                n1 -= 1;
                patch.push(('R', n1, c1[n1]));
            } else if *action == 'I' {
                n1 -= 1;
                n2 -= 1;
            } else {
                unreachable!("can't be!");
            }
        }

        for (action, pos, c) in &patch {
            println!("{action}, {pos}, {c}");
        }

        return patch;
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

fn trace_cache(tables: &Vec<Vec<usize>>, actions: &Vec<Vec<char>>, text1: &str, text2: &str) {
    let c2 = text2.chars().collect::<Vec<char>>();
    let c1 = text1.chars().collect::<Vec<char>>();
    let mut s = String::new();
    for c in c2 {
        s.push_str(&format!("{c}    "));
    }
    println!("        {s}");

    for row in 0..tables.len() {
        for col in 0..tables[row].len() {
            if row != 0 && col == 0 {
                print!("{c}", c = c1[row - 1]);
            }
            print!(
                " {value}({action})",
                value = tables[row][col],
                action = actions[row][col]
            );
        }
        println!();
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

    #[test]
    fn compute_lev_ok() {
        // arrange
        let text1 = "add".to_string();
        // let text2 = "daddy".to_string();
        let text2 = "adyd".to_string();
        let expected = vec![('A', 2, 'y')];

        // act
        let actual = Differ::compute_lev(text1, text2);

        // assert
        assert_eq!(expected, actual);
    }
}
