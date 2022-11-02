use diff_tool::compute_lcs_len_dp;

fn main() {
    let s1: Vec<char> = String::from("abc").chars().collect();
    let s2: Vec<char> = String::from("abcd").chars().collect();
    let r = compute_lcs_len_dp(&s1, &s2);

    println!();

    print!("{} {} ", 0, 0);
    for i in s1 {
        print!("{} ", i);
    }

    println!();

    for i in 0..r.len() {
        if i == 0 {
            print!("{} ", 0);
        } else {
            print!("{} ", &s2[i - 1]);
        }

        for j in 0..r[i].len() {
            print!("{} ", r[i][j]);
        }

        println!();
    }
}
