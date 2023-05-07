/*
Based off demo code from: https://docs.rs/csv/latest/csv/
Start from: https://crates.io/crates/csv
 */

#![allow(dead_code)]
use std::{error::Error, fs};
use std::fs::File;

use serde::Deserialize;
use serde::Serialize;
use serde_json;  // cargo add serde_json

// By default, struct field names are deserialized based on the position of
// a corresponding field in the CSV data's header record.


/*
The error comes from not importing the serde package with the derive feature.
    `Deserialize` is imported here, but it is only a trait, without a derive macro
need to:
    $ cargo add serde --features derive
 */
// #[derive(Debug, Deserialize)]

/* The data file has a header: show,season,episode_n,season_n,title,director,writer,air_date
csv::Reader:: wants te struct built after those names by default.
Flat Episode has the show and season atributes repeated on every object
*/
#[derive(Serialize, Deserialize, Debug)]
struct FlatEpisode {
    show: String,
    season: String,
    episode_n: String,
    season_n: String,
    title: String,
    director: String,
    writer: String,
    air_date: String,  // leaving this as string.  I'll deal with it in Swift if I need to.
}

// struct Episode {
//     episode_n: String,
//     season_n: String,
//     title: String,
//     director: String,
//     writer: String,
//     air_date: String,  // leaving this as string.  I'll deal with it in Swift if I need to.
// }
// my notes on clone:
// https://docs.google.com/document/d/1TFMm8GtPS6t84GEZ6EcA8FJpwrAdjPcTwBqPkvoqk5E/edit#heading=h.yufsze4y63me


// original example
    // fn example() -> Result<(), Box<dyn Error>> {
    //     let mut rdr = csv::Reader::from_reader(io::stdin());
    //     for result in rdr.deserialize() {
    //         // Notice that we need to provide a type hint for automatic
    //         // deserialization.
    //         let record: Record = result?;
    //         println!("{:?}", record);
    //     }
    //     Ok(())
    // }


/*
This returns a result with a vector of strings that are JSON objects.
see improved versions.
 */
fn deserialize_flat_episodes1(opened_file: File) -> Result<Vec<String>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(opened_file);
    let mut flat_episodes_vjs: Vec<String> =  Vec::new(); // vjs short for vector of json strings
    for csv_result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let flat_episode: FlatEpisode = csv_result?;
        let json_flat_eposode_str = serde_json::to_string(&flat_episode)?;
        flat_episodes_vjs.push(json_flat_eposode_str);
    }
    Ok(flat_episodes_vjs)
}

/*
This returns a result with a vector FlatEpisode structs.
That vector should be serialized to a JSON List
 */
fn deserialize_flat_episodes2(opened_file: File) -> Result<Vec<FlatEpisode>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(opened_file);
    let mut flat_episodes: Vec<FlatEpisode> =  Vec::new();
    for csv_result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let flat_episode: FlatEpisode = csv_result?;
        flat_episodes.push(flat_episode);
    }
    Ok(flat_episodes)
}


// fn write_file() -> Result<(), Box<dyn Error>>  {
//     // file writing example from:
//     // https://doc.rust-lang.org/nightly/rust-by-example/std_misc/file/create.html?highlight=write#create
//     let path = Path::new("streamdata.json");
//     let display = path.display();
//
//     // Open a file in write-only mode, returns `io::Result<File>`
//     let mut file = match File::create(&path) {
//         Err(why) => panic!("couldn't create {}: {}", display, why),
//         Ok(file) => file,
//     };
//
//     // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
//     match file.write_all(LOREM_IPSUM.as_bytes()) {
//         Err(why) => panic!("couldn't write to {}: {}", display, why),
//         Ok(_) => println!("successfully wrote to {}", display),
//     }
// }



fn main () -> Result<(), Box<dyn Error>> {
    println!("Program to parse CSV TV Steaming data to JSON for use by Swift project");
    // orig example
        // if let Err(err) = example() {
        //     println!("error running example: {}", err);
        //     process::exit(1);
        // }
    // from: csc363-20223-sonspb6d/wk03/wksheet03/wks3-file/src/main.rs
    // get file from command line:
        // let args : Vec<String> = env::args ().skip (1).collect ();
        // let input_filename : &str = &args[0];
    // let input_filename = "stream-small-data.csv";
    let input_filename = "streamdata.csv";
    println!("Reading from: {input_filename}");
    let output_filename = "streamdata.json";
    println!("Writing to: {output_filename}");
    // let data : String = std::fs::read_to_string (input_filename)?;
    let opened_file = File::open(input_filename)?;

    // first cut resul;ted in the file having multiple top level json objects:
        // let flat_episodes_json = deserialize_flat_episodes1(opened_file)?;
        // for f_e_json in flat_episodes_json.as_slice() {
        //     println!("{f_e_json}")
        // }
        // fs::write("streamdata.json", flat_episodes_json.join("\n"));
        // println!("{flat_episodes_json}");

    let flat_episodes_vf= deserialize_flat_episodes2(opened_file)?;
    let flat_episode_json_array = serde_json::to_string(&flat_episodes_vf)?;
    fs::write(output_filename, flat_episode_json_array);

    Ok (())
}