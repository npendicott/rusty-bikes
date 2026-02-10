use std::fs;
use std::error::Error;
use std::path::Path;
use std::ffi::OsStr;
use std::env;

// use serde_xml_rs;  // TODO: Move to subpackage?

mod capital_bikes;

// Consts
const LIMIT_DEV_RUNS: bool = true;  // TODO: Make an arg?

// Paths
const LOCAL_INDEX_FILEPATH: &str = "./data/index.xml";
const LOCAL_HISTORY_DIRECTORY: &str = "./data/historic/raw";
const LOCAL_HISTORY_UNZIP_DIRECTORY: &str = "./data/historic/unzipped";
// Unzip Test
const LOCAL_HISTORY_UNZIP_TEST_DIRECTORY: &str = "./data/historic/unzipped_test";
const LOCAL_HISTORY_PARQUET_DIRECTORY: &str = "./data/historic/parquet";


// Actions
fn pull_historic() -> Result<(), Box<dyn Error>> {
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
    // let index_result_serialized = serde_xml_rs::to_string(&index_result)?;
    let index_result_serialized = capital_bikes::serialize_bikeshare_history_index(&index_result)?;//  serde_xml_rs::to_string()?;    
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

        // Get the file from API
        let historic_file_path = local_history_directory.join(&historic_file.key);  // TODO: Is join the best here?

        // Check for zip extension and skip any non-zips        
        match historic_file_path.extension().and_then(OsStr::to_str) {
            Some("zip") => {
                println!("{} is a zip.", historic_file_path.display());
            }
            Some(ext) => {
                println!("{} is not a zip: {}.", historic_file_path.display(), ext);
                continue;
            }
            None => {
                println!("{} has no file extension.", historic_file_path.display());
                continue;
            }
        }

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
        if LIMIT_DEV_RUNS {
            if i == 1 {
                break;
            }
        }
    }
    

    Ok(())
}

fn process_csv() -> Result<(), Box<dyn Error>> {
    // Initialize paths and ensure local directories
    let local_history_unzip_test_directory = Path::new(LOCAL_HISTORY_UNZIP_TEST_DIRECTORY);

    let local_history_parquet_directory = Path::new(LOCAL_HISTORY_PARQUET_DIRECTORY);
    fs::create_dir_all(local_history_parquet_directory)?;

    // Iterate CSV File Paths
    let csv_file_paths = fs::read_dir(local_history_unzip_test_directory)?;
    for file_path_result in csv_file_paths {
        let file_path = file_path_result.unwrap().path();
        println!("{}", file_path.display());

        // Read the CSV        
        let file = fs::File::open(file_path)?;
        let mut reader = csv::Reader::from_reader(file);
        for result in reader.records() {
            let record = result?;  //.expect("a CSV record");
            println!("{:?}", record);
        }
    }

    Ok(())
}


// Argument Structure
struct Arguments {
    action: String,
    test: bool,
}

fn parse_args(args: &[String]) -> Arguments {
    let action = args[1].clone();
    // Test?
    // https://stackoverflow.com/questions/49886160/why-can-i-compare-a-string-to-a-str-using-if-but-not-when-using-match
    let test: bool;
    match args[2].as_str() {
        "--test" => test = true,
        _ => test = false,
    }


    Arguments { action, test }
}


// Main
// TODO: Copy this main format
// fn main() {
//     if let Err(err) = run() {
//         println!("{}", err);
//         process::exit(1);
//     }
// }
fn main() -> Result<(), Box<dyn Error>> {
    // Parse Args
    let args: Vec<String> = env::args().collect();
    let arguments: Arguments = parse_args(&args);

    println!("We want to: {0}", arguments.action);
    println!("Test? {0}", arguments.test);

    // Match to what is happening
    match arguments.action.as_str() {
        "pull_historic" => pull_historic()?,
        "process_csv" => process_csv()?,
        _ => println!("No action matched"),
    }

    Ok(())
}
