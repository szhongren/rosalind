use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::ops::Index;

fn main() {
    let mut dataset_path = PathBuf::from(env::current_dir()
        .unwrap_or(PathBuf::from(env!("CARGO_MANIFEST_DIR"))));
    dataset_path.push("datasets");
    dataset_path.push("rosalind_prot.txt");

    let data = read_file(dataset_path);
    print!("{}", prot(data));
}

fn read_file(path: PathBuf) -> String {
    let mut data = String::new();
    let _ = File::open(path)
        .unwrap()
        .read_to_string(&mut data);
    data
}

fn prot(rna: String) -> String {
    let mut ans = String::new();
    let mut start = 0;
    while start < rna.len() {
        ans.push(
            match rna.index(start..start + 3) {
                "UUU" => 'F',
                "UUC" => 'F',
                "UUA" => 'L',
                "UUG" => 'L',
                "UCU" => 'S',
                "UCC" => 'S',
                "UCA" => 'S',
                "UCG" => 'S',
                "UAU" => 'Y',
                "UAC" => 'Y',
                "UAA" => break,
                "UAG" => break,
                "UGU" => 'C',
                "UGC" => 'C',
                "UGA" => break,
                "UGG" => 'W',
                "CUU" => 'L',
                "CUC" => 'L',
                "CUA" => 'L',
                "CUG" => 'L',
                "CCU" => 'P',
                "CCC" => 'P',
                "CCA" => 'P',
                "CCG" => 'P',
                "CAU" => 'H',
                "CAC" => 'H',
                "CAA" => 'Q',
                "CAG" => 'Q',
                "CGU" => 'R',
                "CGC" => 'R',
                "CGA" => 'R',
                "CGG" => 'R',
                "AUU" => 'I',
                "AUC" => 'I',
                "AUA" => 'I',
                "AUG" => 'M',
                "ACU" => 'T',
                "ACC" => 'T',
                "ACA" => 'T',
                "ACG" => 'T',
                "AAU" => 'N',
                "AAC" => 'N',
                "AAA" => 'K',
                "AAG" => 'K',
                "AGU" => 'S',
                "AGC" => 'S',
                "AGA" => 'R',
                "AGG" => 'R',
                "GUU" => 'V',
                "GUC" => 'V',
                "GUA" => 'V',
                "GUG" => 'V',
                "GCU" => 'A',
                "GCC" => 'A',
                "GCA" => 'A',
                "GCG" => 'A',
                "GAU" => 'D',
                "GAC" => 'D',
                "GAA" => 'E',
                "GAG" => 'E',
                "GGU" => 'G',
                "GGC" => 'G',
                "GGA" => 'G',
                "GGG" => 'G',
                _ => '*'
            }
        );
        start += 3;
    }
    ans
}
