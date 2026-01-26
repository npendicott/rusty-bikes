// use std::cmp::Ordering;
// use std::io;
use std::fs;
use std::error::Error;
// use std::io::Result; // Use std::io::Result for convenience --> What does this do??

use serde_xml_rs;


mod capital_bikes;

const LOCAL_INDEX_FILEPATH: &str = "data/index.xml";


fn main() -> Result<(), Box<dyn Error>> {
    // Get the historic ride index
    let index_result = capital_bikes::get_bikeshare_history_index()?;
    
    // // ? -> Match
    // match index_result {
    //     Ok(index) => println!("body = {index}"),
    //     Err(error) => panic!("Ah!"),
    // }

    println!("Got index for: {}.", index_result.name);
    println!("{} files found in bucket.", index_result.contents.len());



    // Write the index to a file
    let index_str = serde_xml_rs::to_string(&index_result).unwrap();
    fs::write(LOCAL_INDEX_FILEPATH, index_str)?;
    println!("Wrote content to {}", LOCAL_INDEX_FILEPATH);


    Ok(())
}
