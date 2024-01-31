use std::{
    sync::{mpsc},
    thread,
    time::Instant
};
use clap::Parser;

const ALPHA: &str = "abcdefghijklmnopqrstuvwxyz";

#[derive(Parser, Debug)]
#[command(about)]
struct Arguments {
    #[arg(short = 'l', long)]
    target_len: Option<usize>,
    #[arg(short = 't', long)]
    threads_num: Option<usize>,
}

fn multithreaded_continue(
    current: String, 
    chars: &String,
    tx: mpsc::Sender<String>,
    target_len: &usize,
) {
    if current.len() == *target_len {
        tx.send(current).unwrap();
        return;
    }

    for ch in ALPHA.chars() {
        multithreaded_continue(format!("{current}{ch}"), chars,tx.clone(), &target_len);
    }
}

fn multithreaded_start(threads_num: usize, target_len: &usize) -> Option<Vec<String>> {
    let size = ALPHA.len() / threads_num;
    let mut threads: Vec<_> = Vec::new();

    let (tx, rx) = mpsc::channel();

    for threaded_chars in ALPHA
        .chars()
        .collect::<Vec<char>>()
        .chunks(size)
        .map(|ch| ch.iter().collect::<String>())
        .collect::<Vec<String>>()
    {
        let tx = tx.clone();
        let target_len = target_len.clone();
        let thread = thread::spawn(move || {
            println!("Created new thread with string: {:?}", threaded_chars);
            for thread_char in threaded_chars.chars() {
                let tx = tx.clone();
                multithreaded_continue(format!("{thread_char}"), &threaded_chars, tx, &target_len);
            }
            println!("Thread with string {threaded_chars}: finished");
            
        });
        threads.push(thread);
    }

    drop(tx);

    // let mut result = vec![];
    let mut count = 0u64;
    while let Some(r) = rx.iter().next() {
        // println!("Received: {r}");
        // result.push(r);
        count += 1;
    }
    println!("Combinations found: {}/{}", count, usize::pow(26, (*target_len).try_into().unwrap()));
    None
}

fn main() {
    let start = Instant::now();

    let arguments = Arguments::parse();
    let threads_num = arguments.threads_num.unwrap_or(1usize);
    let target_len = arguments.target_len.unwrap_or(5usize);

    println!("Combinations of length {target_len} with {threads_num} thread(s)");

    multithreaded_start(threads_num, &target_len);

    println!("Elapsed: {:?}\n##############", start.elapsed());
}
