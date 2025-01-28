extern crate flate2;
use flate2::write::GzEncoder;
use flate2::Compression;
//accept the name  of file
use std::env::args;
//imput the fiel
use std::fs::File;
//copy the file
use std::io::copy;
//read the file
use std::io::BufReader;
use std::net;
use std::process::Output;
//when we want to show time to compress fiel
use std::time::Instant;

fn main() {
    //taking 3 arguments
    if args().len() != 3 {
        eprintln!("Usage:'Source''target'");
        return;
    }
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let output = File::create(args().nth(2).unwrap()).unwrap();
    let mut Encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();
    copy(&mut input, &mut Encoder);
    let output = Encoder.finish().unwrap();
    println!("source{:?}", input.get_ref().metadata().unwrap().len());
    println!("Elapse{:?}", start.elapsed());
}
