use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

fn main() {
    let mut dataset_path = PathBuf::from(env::current_dir()
        .unwrap_or(PathBuf::from(env!("CARGO_MANIFEST_DIR"))));
    dataset_path.push("datasets");
    dataset_path.push("rosalind_rna.txt");

    let mut data = String::new();
    let _ = File::open(dataset_path)
        .unwrap()
        .read_to_string(&mut data);
    print!("{}", data.replace("T", "U"));
}
