pub struct Differ {
    pub text1: String,
    pub text2: String,
}

impl Differ {
    pub fn new(text1: String, text2: String) -> Self {
        Differ { text1, text2 }
    }

    fn diff_by_lev(text1: String, text2: String) -> Vec<(char, usize, char)> {
        let c1 = text1.chars().collect::<Vec<char>>();
        let c2 = text2.chars().collect::<Vec<char>>();
        let n = text1.len();
        let m = text2.len();
        let mut table = vec![vec![0; m + 1]; n + 1];
        table[0][0] = 0;
        let mut actions = vec![vec!['-'; m + 1]; n + 1];
        actions[0][0] = 'I';

        for i in 1..n + 1 {
            actions[i][0] = 'R';
            table[i][0] = i;
        }

        for j in 1..m + 1 {
            actions[0][j] = 'A';
            table[0][j] = j;
        }

        for i in 1..n + 1 {
            for j in 1..m + 1 {
                if c1[i - 1] == c2[j - 1] {
                    actions[i][j] = 'I';
                    table[i][j] = table[i - 1][j - 1];
                    continue;
                }

                let remove = table[i - 1][j];
                let add = table[i][j - 1];

                table[i][j] = remove;
                actions[i][j] = 'R';

                if add < table[i][j] {
                    table[i][j] = add;
                    actions[i][j] = 'A';
                }

                table[i][j] += 1;
            }
        }

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

    fn compute_lcs_table<T>(text1: &Vec<T>, text2: &Vec<T>) -> Vec<Vec<i32>>
    where
        T: PartialEq,
    {
        let n = text1.len();
        let m = text2.len();
        let mut table = vec![vec![0; m + 1]; n + 1];

        for i in 0..n + 1 {
            for j in 1..m + 1 {
                if i == 0 || j == 0 {
                    table[i][j] = 0;
                } else if text1[i - 1] == text2[j - 1] {
                    table[i][j] = table[i - 1][j - 1] + 1
                } else {
                    table[i][j] = std::cmp::max(table[i - 1][j], table[i][j - 1])
                }
            }
        }

        return table;
    }

    fn diff_by_lcs<T>(text1: &Vec<T>, text2: &Vec<T>) -> Vec<(char, usize, T)>
    where
        T: PartialEq + Copy + Clone,
    {
        let mut n = text1.len();
        let mut m = text2.len();
        let table = Self::compute_lcs_table(text1, text2);

        let mut patch: Vec<(char, usize, T)> = Vec::new();

        while n != 0 || m != 0 {
            if n == 0 {
                patch.push(('+', m - 1, text2[m - 1]));
                m -= 1;
            } else if m == 0 {
                patch.push(('-', n - 1, text1[n - 1]));
                n -= 1;
            } else if text1[n - 1] == text2[m - 1] {
                n -= 1;
                m -= 1;
            } else if table[n - 1][m] <= table[n][m - 1] {
                patch.push(('+', m - 1, text2[m - 1]));
                m -= 1;
            } else if table[n - 1][m] >= table[n][m - 1] {
                patch.push(('-', n - 1, text1[n - 1]));
                n -= 1;
            } else {
                unreachable!("Can't be here");
            }
        }

        patch.reverse();
        return patch;
    }

    // pub fn diff_by_chars(&self) -> Vec<String> {
    //     let new_text = self
    //         .new_text
    //         .chars()
    //         .map(|x| x.to_string())
    //         .collect::<Vec<String>>();
    //     let old_text = self
    //         .old_text
    //         .chars()
    //         .map(|x| x.to_string())
    //         .collect::<Vec<String>>();
    //
    //     return Self::compute_lcs(&old_text, &new_text);
    // }

    // pub fn diff_by_words(&self) -> Vec<String> {
    //     let text2 = self
    //         .text1
    //         .split_ascii_whitespace()
    //         .map(|x| x.to_string())
    //         .collect::<Vec<String>>();
    //
    //     let text1 = self
    //         .text2
    //         .split_ascii_whitespace()
    //         .map(|x| x.to_string())
    //         .collect::<Vec<String>>();
    //
    //     return Self::compute_lcs(&text1, &text2);
    // }
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

    // #[test]
    // fn diff_test_chars_ok_1() {
    //     let new = String::from("abcd");
    //     let old = String::from("abc");
    //     let content = Differ {
    //         text1: new,
    //         text2: old,
    //     };
    //     let actual = content.diff_by_chars();
    //
    //     let expected = vec!["a", "b", "c", "+d"];
    //     for i in 0..actual.len() {
    //         assert_eq!(actual[i], expected[i]);
    //     }
    // }

    // #[test]
    // fn diff_test_chars_ok_2() {
    //     let new = String::from("abcd");
    //     let old = String::from("");
    //     let content = Differ {
    //         text1: new,
    //         text2: old,
    //     };
    //     let actual = content.diff_by_chars();
    //
    //     let expected = vec!["+a", "+b", "+c", "+d"];
    //     for i in 0..actual.len() {
    //         assert_eq!(actual[i], expected[i]);
    //     }
    // }

    // #[test]
    // fn diff_test_chars_ok_3() {
    //     let new = String::from("");
    //     let old = String::from("abcd");
    //     let content = Differ {
    //         text1: new,
    //         text2: old,
    //     };
    //     let actual = content.diff_by_chars();
    //
    //     let expected = vec!["-a", "-b", "-c", "-d"];
    //     for i in 0..actual.len() {
    //         assert_eq!(actual[i], expected[i]);
    //     }
    // }

    // #[test]
    // fn diff_test_chars_ok_4() {
    //     let new = String::from("abecd");
    //     let old = String::from("zaabck");
    //     let content = Differ {
    //         text1: new,
    //         text2: old,
    //     };
    //     let actual = content.diff_by_chars();
    //
    //     let expected = vec!["-z", "-a", "a", "b", "+e", "c", "-k", "+d"];
    //     for i in 0..actual.len() {
    //         assert_eq!(actual[i], expected[i]);
    //     }
    // }

    // #[test]
    // fn diff_test_words_ok_1() {
    //     let new = String::from("He is Andy");
    //     let old = String::from("She is Amy");
    //     let content = Differ {
    //         text1: new,
    //         text2: old,
    //     };
    //
    //     let actual = content.diff_by_words();
    //     let expected = vec!["-She", "+He", "is", "-Amy", "+Andy"];
    //     for i in 0..actual.len() {
    //         assert_eq!(actual[i], expected[i]);
    //     }
    // }

    // #[test]
    // fn diff_test_words_ok_2() {
    //     let new = String::from("He is Andy");
    //     let old = String::from("He is");
    //     let content = Differ {
    //         text1: new,
    //         text2: old,
    //     };
    //
    //     let actual = content.diff_by_words();
    //     let expected = vec!["He", "is", "+Andy"];
    //     for i in 0..actual.len() {
    //         assert_eq!(actual[i], expected[i]);
    //     }
    // }

    // #[test]
    // fn diff_test_words_ok_3() {
    //     let new = String::from("is Andy");
    //     let old = String::from("He is Andy");
    //     let content = Differ {
    //         text1: new,
    //         text2: old,
    //     };
    //
    //     let actual = content.diff_by_words();
    //     let expected = vec!["-He", "is", "Andy"];
    //     for i in 0..actual.len() {
    //         assert_eq!(actual[i], expected[i]);
    //     }
    // }

    // #[test]
    // fn diff_test_words_ok_4() {
    //     let new = String::from("He is Andy She is Anne");
    //     let old = String::from("He is Andy I am Amy");
    //     let content = Differ {
    //         text1: new,
    //         text2: old,
    //     };
    //
    //     let actual = content.diff_by_words();
    //     println!("actual: {:?}", actual);
    //     let expected = vec![
    //         "He", "is", "Andy", "-I", "-am", "-Amy", "+She", "+is", "+Anne",
    //     ];
    //     for i in 0..actual.len() {
    //         assert_eq!(actual[i], expected[i]);
    //     }
    // }

    // #[test]
    // fn diff_test_words_ok_5() {
    //     let new = String::from("He is Andy I am Amyyy She is Anne");
    //     let old = String::from("He is Andy I am Amy");
    //     let content = Differ {
    //         text1: new,
    //         text2: old,
    //     };
    //
    //     let actual = content.diff_by_words();
    //     let expected = vec![
    //         "He", "is", "Andy", "I", "am", "-Amy", "+Amyyy", "+She", "+is", "+Anne",
    //     ];
    //     for i in 0..actual.len() {
    //         assert_eq!(actual[i], expected[i]);
    //     }
    // }

    #[test]
    fn compute_lev_ok() {
        // arrange
        let text1 = "add".to_string();
        // let text2 = "daddy".to_string();
        let text2 = "adyd".to_string();
        let expected = vec![('A', 2, 'y')];

        // act
        let actual = Differ::diff_by_lev(text1, text2);

        // assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn compute_lcs_ok_1() {
        // arrange
        let text1 = "add".to_string();
        let text2 = "adyd".to_string();
        let expected = vec![('+', 2, 'y')];

        // act
        let actual = Differ::diff_by_lcs(&text1.chars().collect(), &text2.chars().collect());

        // assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn compute_lcs_ok_2() {
        // arrange
        let text1 = "add".to_string();
        let text2 = "add".to_string();
        let expected: Vec<(char, usize, char)> = Vec::new();

        // act
        let actual = Differ::diff_by_lcs(&text1.chars().collect(), &text2.chars().collect());

        // assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn compute_lcs_ok_3() {
        // arrange
        let text1 = "addy".to_string();
        let text2 = "add".to_string();
        let expected: Vec<(char, usize, char)> = vec![('-', 3, 'y')];

        // act
        let actual = Differ::diff_by_lcs(&text1.chars().collect(), &text2.chars().collect());

        // assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn compute_lcs_ok_4() {
        // arrange
        let text1 = "yadd".to_string();
        let text2 = "add".to_string();
        let expected: Vec<(char, usize, char)> = vec![('-', 0, 'y')];

        // act
        let actual = Differ::diff_by_lcs(&text1.chars().collect(), &text2.chars().collect());

        // assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn compute_lcs_ok_5() {
        // arrange
        let text1 = "ccc".to_string();
        let text2 = "add".to_string();
        let expected: Vec<(char, usize, char)> = vec![
            ('-', 0, 'c'),
            ('-', 1, 'c'),
            ('-', 2, 'c'),
            ('+', 0, 'a'),
            ('+', 1, 'd'),
            ('+', 2, 'd'),
        ];

        // act
        let actual = Differ::diff_by_lcs(&text1.chars().collect(), &text2.chars().collect());

        // assert
        assert_eq!(expected, actual);
    }
}
