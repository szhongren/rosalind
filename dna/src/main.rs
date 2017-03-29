use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::collections::HashMap;

fn main() {
    let mut dataset_path = PathBuf::from(env::current_dir()
        .unwrap_or(PathBuf::from(env!("CARGO_MANIFEST_DIR"))));
    dataset_path.push("datasets");
    dataset_path.push("rosalind_dna.txt");

    let mut data = String::new();
    let _ = File::open(dataset_path).unwrap().read_to_string(&mut data);
    let mut counts: HashMap<char, i64> = HashMap::new();
    for ch in data.trim().chars() {
        *counts.entry(ch)
            .or_insert(0) += 1;
    }

    for ch in "ACGT".chars() {
        print!("{} ", counts.get(&ch).unwrap_or(&-1));
    }
}
