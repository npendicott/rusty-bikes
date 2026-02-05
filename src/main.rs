use std::fs;
use std::error::Error;
// use std::io::Result; // Use std::io::Result for convenience --> What does this do??
use std::path::Path;

use serde_xml_rs;


mod capital_bikes;

const LOCAL_INDEX_FILEPATH: &str = "./data/index.xml";
const LOCAL_HISTORY_DIRECTORY: &str = "./data/historic/raw";
const LOCAL_HISTORY_UNZIP_DIRECTORY: &str = "./data/historic/unzipped";

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize paths and ensure local directories
    let local_index_filepath = Path::new(LOCAL_INDEX_FILEPATH);
    let local_index_directory = local_index_filepath.parent().unwrap();
    fs::create_dir_all(local_index_directory)?;

    let local_history_directory = Path::new(LOCAL_HISTORY_DIRECTORY);
    fs::create_dir_all(local_history_directory)?;

    let local_history_unzip_directory = Path::new(LOCAL_HISTORY_UNZIP_DIRECTORY);
    fs::create_dir_all(local_history_unzip_directory)?;

    // Get the historic ride index
    let index_result = capital_bikes::get_bikeshare_history_index()?;
    println!("{} files found in bucket.", index_result.contents.len());

    // Write the index to a file
    let index_result_serialized = serde_xml_rs::to_string(&index_result)?;
    fs::write(local_index_filepath, index_result_serialized)?;
    println!("Cached index to {}", local_index_filepath.display());

    // Iterate files
    println!("\nIterating files:");
    let mut i: i32 = 0;
    for historic_file in index_result.contents {
        // // Check Output
        // println!("key: {}", historic_file.key);
        // println!("last_modified: {}", historic_file.last_modified);
        // println!("e_tag: {}", historic_file.e_tag);
        // println!("size: {}", historic_file.size);
        // println!("storage_class: {}", historic_file.storage_class); 
        // println!(""); 

        // TODO: Check for zip extension - After FP?
        // Get the file from API
        let historic_file_path = local_history_directory.join(&historic_file.key);  // TODO: Is join the best here?
        let historic_file_contents = capital_bikes::get_bikeshare_history_file(&historic_file)?;

        // Write file to disk
        fs::write(&historic_file_path, historic_file_contents)?;
        println!("Wrote historic file to {}", historic_file_path.display());

        // Unzip - TODO: Can I just unzup the Bytes?
        // TODO: If I'm not unzipping the Bytes, then it probably makese sense to unzip in a seperate loop?
        if let Err(e) = capital_bikes::unzip_file(&historic_file_path, local_history_unzip_directory) {
            eprintln!("Error unzipping file: {}", e);
        }

        // Limit runs for testing
        i = i + 1;
        if i == 1 {
            break;
        }
    }
    

    Ok(())
}
