use csv;
use std::error::Error;

fn main() {
    if let Err(e) = real_main("./customers.csv") {
        eprintln!("{}", e)
    }
}

fn real_main(path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(&path)?;

    for result in reader.records() {
        let record = result?;

        // let name = &record[2];

        // println!("{}", name)

        println!("{:?}", record);
    }

    Ok(())
}
