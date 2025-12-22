use flate2::Compression;
use flate2::write::GzEncoder;
use std::env::args;
use std::fs::File;
use std::io::BufReader;
use std::io::copy;
use std::time::Instant;

fn main() {
    if args().len() != 3 {
        eprintln!("Usage `source` `target`");
        return;
    };

    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let output = File::create(args().nth(2).unwrap()).unwrap();
    let mut encode = GzEncoder::new(output, Compression::default());

    copy(&mut input, &mut encode).unwrap();
    let output = encode.finish().unwrap();
    let start = Instant::now();

    println!(
        "Source size: {:?}",
        input.get_ref().metadata().unwrap().len()
    );

    println!("Target size: {:?}", output.metadata().unwrap().len());

    println!("Time taken: {:?}", start.elapsed());
}
