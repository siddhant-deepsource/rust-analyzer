// go to /code
// install all deps there
// run clippy there
// get the output in a file
// serialize that to json
// write it to analysis_results.toml
// calculate doc coverage https://crates.io/crates/cargo-doc-coverage
// count no. of deps by reading Cargo.toml? or any other way
// write everything to analysis_results.toml
// publish

use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::process::Command;

struct AnalyzerOpts {
    Name: String,
    AnalysisConfigPath: String,
    FileFinderRegex: String,
    CodePath: String,
    ResultPath: String,
}

#[derive(Serialize, Deserialize)]
struct Report {
    reason: String,
    message: Message,
}

struct Message {
    code: Code,
}

struct Code {
    code: String,
    level : String,
    message: String,
    spans: 
}
fn main() {
    // instantiating analyzer opts
    let analyzer_opts = AnalyzerOpts {
        Name: String::from("javascript"),
        AnalysisConfigPath: String::from("/toolbox/analysis_config.json"),
        FileFinderRegex: String::from("\\.js$"),
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

    // println!("{}", String::from_utf8_lossy(&output.stdout));
    // write the output to a file
    let mut buffer = File::create("foo.txt").unwrap();
    buffer.write_all(&output.stdout);
    // read by line
    let mut v = Vec::new();

    if let Ok(lines) = read_lines("./foo.txt") {
        for line in lines {
            if let Ok(ip) = line {
                // println!("{}", ip);
                let _res: Report = serde_json::from_str(&ip).unwrap();
                println!("{} hello", _res.reason);
                if (_res.reason == "compiler-message") {
                    v.push(_res)
                }
            }
        }
    }

    for i in &v {
        println!("{}", i.reason);
    }

    // and make an array of objects
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
