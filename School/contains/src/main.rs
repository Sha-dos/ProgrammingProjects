use std::fmt::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use rand::prelude::*;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 { panic!("expected 1 argument") }

    let phrase = &args[1];
    let filename = "./src/static/words.txt";

    let lines = lines_from_file(filename).await;

    let ans = find(&&lines, phrase.to_string()).await;
    println!("Ans: {:?}", ans);

    Ok(())
}

async fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
} 
async fn find(lines: &Vec<String>, phrase: String) -> String {
    let mut ans: Vec<String> = vec![];
    for line in lines {
        if line.contains(&phrase) { ans.push(line.to_string()); }
    }
    let rand_num: usize = rand::thread_rng().gen_range(0..ans.len());
    return ans.into_iter().nth(rand_num).unwrap();
}
