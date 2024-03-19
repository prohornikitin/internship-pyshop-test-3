mod args;

use std::{ops::Range, thread::{self, JoinHandle}};

use args::Args;
use clap::Parser;
use regex::Regex;
use sha256;



fn has_exatly_n_trailing_zeros(n: usize) -> Regex {
    Regex::new(format!(r"0{{{}}}$", n).as_str()).unwrap()
}


#[derive(Clone, Debug)]
struct HashCase {
    input: usize,
    digest: String,
}

fn spawn_worker_thread(inputs: Range<usize>, regex: Regex) -> JoinHandle<Vec<HashCase>> {
    thread::spawn(move|| {
        let mut results: Vec<HashCase> = vec![];
        for i in inputs {
            let digest = sha256::digest(i.to_string());
            if regex.is_match(digest.as_str()) {
                results.push(HashCase {
                    input: i,
                    digest: digest,
                });
            }
        }
        return results;
    })
}


fn main() {
    let args = Args::parse();

    let mut new_chunk_start = 1;
    let mut found = 0;
    let regex = has_exatly_n_trailing_zeros(args.trailing_zeros);
    while new_chunk_start < usize::MAX && found < args.hashes_needed  {
        let mut handles: Vec<JoinHandle<Vec<HashCase>>> = vec![];
        for _ in 0..args.threads {
            handles.push(spawn_worker_thread(
                new_chunk_start..new_chunk_start + args.chunk_size,
                regex.clone()
            ));
            new_chunk_start += args.chunk_size;
        }

        let mut results: Vec<HashCase> = vec![];
        for h in handles {
            for v in h.join().unwrap() {
                results.push(v);
            }
        }
        results.sort_unstable_by_key(|x| x.input);
        for v in results {
            println!("{}, {}", v.input, v.digest);
            found += 1;
            if found >= args.hashes_needed {
                return;
            }
        }
    }
    
}
