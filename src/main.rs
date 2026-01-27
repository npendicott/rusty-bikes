// use std::cmp::Ordering;
// use std::io;
use std::fs;
use std::error::Error;
// use std::io::Result; // Use std::io::Result for convenience --> What does this do??

use serde_xml_rs;


mod capital_bikes;

const LOCAL_INDEX_FILEPATH: &str = "data/index.xml";
const LOCAL_HISTORY_DIRECTORY: &str = "data/historic/";

fn main() -> Result<(), Box<dyn Error>> {

    // Get the historic ride index
    let index_result = capital_bikes::get_bikeshare_history_index()?;
    println!("{} files found in bucket.", index_result.contents.len());

    // Write the index to a file
    let index_ser = serde_xml_rs::to_string(&index_result)?;
    fs::write(LOCAL_INDEX_FILEPATH, index_ser)?;
    println!("Cached index to {}", LOCAL_INDEX_FILEPATH);

    // Iterate files
    println!("\nIterating files:");
    let mut i: i32 = 0;
    for historic_file in index_result.contents {
        // println!("key: {}", historic_file.key);
        // println!("last_modified: {}", historic_file.last_modified);
        // println!("e_tag: {}", historic_file.e_tag);
        // println!("size: {}", historic_file.size);
        // println!("storage_class: {}", historic_file.storage_class); 

        // println!(""); 

        // Get the file
        let historic_file_path = format!("{LOCAL_HISTORY_DIRECTORY}{0}", historic_file.key);
        let historic_file_contents = capital_bikes::get_bikeshare_history_file(&historic_file)?;
        
        fs::write(&historic_file_path, historic_file_contents)?;
        println!("Wrote historic file to {}", historic_file_path);


        // Limit runs for testing
        i = i + 1;
        if i == 1 {
            break;
        }
    }
    

    Ok(())
}
