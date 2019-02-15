extern crate rayon;

use rayon::prelude::*;
use std::collections::*;
use std::env;
use std::fs;
use std::io;
use std::sync::Mutex;
use std::sync::PoisonError;

#[derive(Debug)]
enum TallyError {
    Lock,
    IO,
}

impl From<io::Error> for TallyError {
    fn from(_: io::Error) -> TallyError {
        TallyError::IO
    }
}

impl<T> From<PoisonError<T>> for TallyError {
    fn from(_: PoisonError<T>) -> TallyError {
        TallyError::Lock
    }
}

fn main() -> Result<(), TallyError> {
    let words = Mutex::new(HashMap::new());

    let res: Result<Vec<()>, TallyError> = env::args()
        .skip(1)
        .collect::<Vec<String>>()
        .par_iter()
        .map(|arg| tally_words(arg.to_string(), &words))
        .collect();
    res?;

    words.lock()?.iter().for_each(|(word, count)| {
        if *count >= 1 {
            println!("{} = {}", word, count);
        }
    });

    Ok(())
}

fn tally_words(filename: String, words: &Mutex<HashMap<String, u32>>) -> Result<(), TallyError> {
    let contents = fs::read_to_string(filename)?;
    for s in contents.split_whitespace() {
        let key = s.to_lowercase();
        *words.lock()?.entry(key).or_insert(0) += 1;
    }
    Ok(())
}
