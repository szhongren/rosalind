use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

fn main() {
    let mut dataset_path = PathBuf::from(env::current_dir()
        .unwrap_or(PathBuf::from(env!("CARGO_MANIFEST_DIR"))));
    dataset_path.push("datasets");
    dataset_path.push("rosalind_fib.txt");

    let (n, k) = read_file(dataset_path);
    print!("{}", fib(n, k));
}

fn read_file(path: PathBuf) -> (u64, u64) {
    let mut data = String::new();
    let _ = File::open(path)
        .unwrap()
        .read_to_string(&mut data);
    let ans: Vec<u64> = data
        .split_whitespace()
        .take(2)
        .map(|v|
            v.to_string()
                .parse::<u64>()
                .unwrap_or_default()
            )
        .collect();
    (ans[0].clone(), ans[1].clone())
}

fn fib(n: u64, k: u64) -> u64 {
    let mut dp: Vec<u64> = vec![0, 1];
    while dp.len() < n as usize + 1 {
        let last = dp[dp.len() - 1];
        let last_2 = dp[dp.len() - 2];
        dp.push(last + k * last_2);
    }
    for v in dp.clone() {
        println!("{}", v);
    }
    *dp.last().unwrap()
    // adults, babies, total
    // 0 1 1
    // 1 0 1
    // 1 3 4
    // 4 3 7
    // 7 12 19
    // 19 21 40
}
