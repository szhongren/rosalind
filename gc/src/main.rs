use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::collections::HashMap;

fn main() {
    let mut dataset_path = PathBuf::from(env::current_dir()
        .unwrap_or(PathBuf::from(env!("CARGO_MANIFEST_DIR"))));
    dataset_path.push("datasets");
    dataset_path.push("rosalind_gc.txt");

    let ans = read_file(dataset_path);
    let (label, percentage) = gc(ans);
    print!("{}\n{}", label, percentage);
}

fn read_file(path: PathBuf) -> HashMap<String, String> {
    let mut data = String::new();
    let _ = File::open(path)
        .unwrap()
        .read_to_string(&mut data);
    let all = data.split_whitespace()
        .map(|l| l.to_string())
        .map(|mut l|
            if l.contains('>') {
                l.push('<');
                l
            } else {
                l
            })
        .fold(String::new(), |mut acc, l|
            {
                acc.push_str(&l);
                acc
            });
    let records = all.split('>')
        .map(|r|
            r.split('<')
                .map(|v| v.to_string())
                .collect::<Vec<String>>())
        .skip(1);
    let mut labelled = HashMap::new();
    for r in records {
        labelled.insert(r[0].clone(), r[1].clone());
    }
    labelled
}

fn gc(genomes: HashMap<String, String>) -> (String, f64) {
    let mut max = 0.0;
    let mut label = String::new();
    for (k, v) in genomes {
        let percentage = count_gc(v);
        if percentage > max {
            max = percentage;
            label = k;
        }
    }
    (label, max * 100.0)
}

fn count_gc(genome: String) -> f64 {
    let mut gc = 0.0;
    for nucleotide in genome.chars() {
        match nucleotide {
            'G' | 'C' => gc += 1.0,
            _ => ()
        }
    }
    gc / genome.len() as f64
}
