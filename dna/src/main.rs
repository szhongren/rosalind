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

    let data = read_file(dataset_path);
    let counts = dna(data);
    print_ans(counts);
}

fn read_file(path: PathBuf) -> String {
    let mut data = String::new();
    let _ = File::open(path)
        .unwrap()
        .read_to_string(&mut data);
    data
}

fn dna(genome: String) -> HashMap<char, i64> {
    let mut res: HashMap<char, i64> = HashMap::new();

    for ch in genome.trim().chars() {
        *res.entry(ch)
            .or_insert(0) += 1;
    }

    res
}

fn print_ans(counts: HashMap<char, i64>) {
    for ch in "ACGT".chars() {
        print!("{} ", counts.get(&ch).unwrap_or(&0));
    }
}
