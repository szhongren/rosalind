use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::ops::Index;

fn main() {
    let mut dataset_path = PathBuf::from(env::current_dir()
        .unwrap_or(PathBuf::from(env!("CARGO_MANIFEST_DIR"))));
    dataset_path.push("datasets");
    dataset_path.push("rosalind_subs.txt");

    let (genome, motif) = read_file(dataset_path);
    let ans = subs(genome, motif);
    for i in ans {
        print!("{} ", i + 1);
    }
}

fn read_file(path: PathBuf) -> (String, String) {
    let mut data = String::new();
    let _ = File::open(path)
        .unwrap()
        .read_to_string(&mut data);
    let ans: Vec<String> = data
        .lines()
        .take(2)
        .map(|v|
            v.to_string())
        .collect();
    (ans[0].clone(), ans[1].clone())
}

fn subs(genome: String, motif: String) -> Vec<usize> {
    let mut ans = Vec::new();
    let mut start = 0;
    while start < genome.len() {
        match genome.index(start..).find(&motif) {
            Some(i) => {
                ans.push(start + i);
                start += i + 1;
            },
            None => break
        }
    }
    ans
}
