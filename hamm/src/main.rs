use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

fn main() {
    let mut dataset_path = PathBuf::from(env::current_dir()
        .unwrap_or(PathBuf::from(env!("CARGO_MANIFEST_DIR"))));
    dataset_path.push("datasets");
    dataset_path.push("rosalind_hamm.txt");

    let (a, b) = read_file(dataset_path);
    print!("{}", hamm(a, b));
}

fn read_file(path: PathBuf) -> (String, String) {
    let mut data = String::new();
    let _ = File::open(path)
        .unwrap()
        .read_to_string(&mut data);
    let ans = data
        .lines()
        .take(2)
        .map(|v|
            v.trim()
            .to_string())
        .collect::<Vec<String>>();
    (ans[0].clone(), ans[1].clone())
}

fn hamm(a: String, b: String) -> u64 {
    a.chars()
        .zip(
            b.chars()
            )
        .map(|v| {
            let (x, y) = v;
            (x != y) as u64
            })
        // .collect::<Vec<u8>>()
        .fold(0, |acc, v| acc + v)
}
