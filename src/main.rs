// go to /code [DONE]
// install all deps there [DONE]
// run clippy there [DONE]
// get the output in a file [DONE]
// serialize that to json [DONE]
// write it to results.json file [DONE]
// calculate doc coverage https://crates.io/crates/cargo-doc-coverage
// count no. of deps by reading Cargo.toml? or any other way
// write everything to analysis_results.toml
// publish

use linecount::count_lines;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::process::Command;
use toml::Value;

// analyzer options
struct AnalyzerOpts {
    Name: String,
    AnalysisConfigPath: String,
    FileFinderRegex: String,
    CodePath: String,
    ResultPath: String,
}

// structs to receive the analyzed data
#[derive(Serialize, Deserialize)]
struct Report {
    reason: String,
    message: Message,
}

#[derive(Serialize, Deserialize)]
struct Message {
    code: Code,
    level: String,
    message: String,
    spans: Vec<Span>,
}

#[derive(Serialize, Deserialize)]
struct Code {
    code: String,
}

#[derive(Serialize, Deserialize)]
struct Span {
    line_start: i32,
    line_end: i32,
}

#[derive(Deserialize)]
struct Dependencies {}

fn main() {
    // instantiating analyzer opts
    let analyzer_opts = AnalyzerOpts {
        Name: String::from("rust"),
        AnalysisConfigPath: String::from("/toolbox/analysis_config.json"),
        FileFinderRegex: String::from("\\.rs$"),
        CodePath: String::from("/Users/sidntrivedi012/Code/cap"),
        ResultPath: String::from("/toolbox/analysis_results.json0"),
    };

    // rustup update
    let output = Command::new("rustup")
        .args(&["update"])
        .current_dir(&analyzer_opts.CodePath)
        .output()
        .expect("ls command failed to start");

    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    assert!(output.status.success());

    // installing clippy
    let output = Command::new("rustup")
        .args(&["component", "add", "clippy"])
        .current_dir(&analyzer_opts.CodePath)
        .output()
        .expect("ls command failed to start");

    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    // running clippy and getting data in json format
    let output = Command::new("cargo")
        .args(&[
            "clippy",
            "--message-format",
            "json",
            "--",
            "-W",
            "clippy::all",
        ])
        .current_dir(&analyzer_opts.CodePath)
        .output()
        .expect("clippy failed to work");

    // io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    // write the output to a file
    let mut buffer = File::create("foo.txt").unwrap();
    buffer.write_all(&output.stdout);
    // reading the clippy out line by line
    let mut v = Vec::new();
    // counting lines to prevent parsing the last 3 objects (they are useless)
    let lines_count: usize = count_lines(std::fs::File::open("foo.txt").unwrap()).unwrap();
    let mut count: usize = 0;

    // iterating throughout the file of clippy data and finding the useful stuff
    // and dumping into the analyzer object
    if let Ok(lines) = read_lines("./foo.txt") {
        for line in lines {
            count = count + 1;
            if count == lines_count - 3 {
                break;
            }

            if let Ok(ip) = line {
                // println!("{}", ip);
                if ip.starts_with("{\"reason\":\"compiler-message\"") {
                    let _res: Report = serde_json::from_str(&ip).unwrap();
                    // println!("{} hello", _res.message.code.code);
                    if _res.reason == "compiler-message" {
                        // and make an array of objects
                        v.push(_res)
                    }
                }
            }
        }
    }

    let mut buffer = File::create("results.json").unwrap();

    let mut out;
    // just a check
    for i in &v {
        out = serde_json::to_string_pretty(&i).unwrap();
        println!("{}", out);
        buffer
            .write_all(&out.as_bytes())
            .expect("Writing the analysis result failed");
    }

    // write the output to a file

    // DEPENDENCY CALCULATION
    // command - cargo tree --prefix depth | grep -c '^[[:space:]]*1' | wc -l
    // count direct dependencies
    // let output = Command::new("cargo")
    //     .args(&["tree", "--prefix", "depth", "--", "-W", "clippy::all"])
    //     .current_dir(&analyzer_opts.CodePath)
    //     .output()
    //     .expect("clippy failed to work");
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
