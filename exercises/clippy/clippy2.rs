// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DON

fn main() {
    let mut res = 42;
    let option = Some(12);
    //res+=12;
    if let Some(x) = option {
        res += x;
    }
    println!("{}", res);
}
