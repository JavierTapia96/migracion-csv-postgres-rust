use std::error::Error;
use std::fs; 

pub fn read() -> Result<(), Box<dyn Error>> {
    let paths = fs::read_dir("./tmp/").unwrap(); 

    for path in paths { 
        let dir = path?;
        let mut rdr = csv::ReaderBuilder::new()
                                        .has_headers(false)
                                        .from_path(dir.path())
                                        .expect("Cant read field");
        for result in rdr.records() {
            let record = result?;
            println!("{:?}", record);
        }
    }

    Ok(())
}