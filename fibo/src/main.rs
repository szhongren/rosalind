use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

fn main() {
    let mut dataset_path = PathBuf::from(env::current_dir()
        .unwrap_or(PathBuf::from(env!("CARGO_MANIFEST_DIR"))));
    dataset_path.push("datasets");
    dataset_path.push("rosalind_fibo.txt");

    let data = read_file(dataset_path);
    print!("{}", fibo(fibo_i(data)));
}

fn read_file(path: PathBuf) -> String {
    let mut data = String::new();
    let _ = File::open(path)
        .unwrap()
        .read_to_string(&mut data);
    data
}

fn fibo_i(data: String) -> u64 {
    data.trim()
        .parse::<u64>()
        .unwrap_or(0)
}

fn fibo(i: u64) -> u64 {
    let mut dp: Vec<u64> = vec![0, 1];
    while dp.len() < i as usize + 1 {
        let last = dp[dp.len() - 1];
        let last_2 = dp[dp.len() - 2];
        dp.push(last + last_2);
    }
    *dp.last().unwrap()
}
