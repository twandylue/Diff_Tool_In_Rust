use diff_tool::{compute_lcs_len_dp, diff_chars};

fn main() {
    let new: Vec<char> = String::from("abcdefghi").chars().collect();
    let old: Vec<char> = String::from("azedbcdz").chars().collect();
    let result = diff_chars(&new, &old);

    // print diff
    for i in result {
        println!("{}", i);
    }

    // print metrix
    let r = compute_lcs_len_dp(&new, &old);

    println!();

    print!("{} {} ", 0, 0);
    for i in new {
        print!("{} ", i);
    }

    println!();

    for i in 0..r.len() {
        if i == 0 {
            print!("{} ", 0);
        } else {
            print!("{} ", &old[i - 1]);
        }

        for j in 0..r[i].len() {
            print!("{} ", r[i][j]);
        }

        println!();
    }
}
