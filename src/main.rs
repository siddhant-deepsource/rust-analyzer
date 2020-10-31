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
use std::io;
use std::io::prelude::Read;
use std::io::{BufRead, Write};
use std::path::Path;
use std::process::{Command, Stdio};
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
        ResultPath: String::from("/toolbox/analysis_results.json"),
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
    let mut result_output = serde_json::to_string_pretty(&v).unwrap();

    // writing the output to a file
    buffer
        .write_all(&result_output.as_bytes())
        .expect("Writing the analysis result failed");

    // DEPENDENCY CALCULATION
    // command - cargo tree --prefix depth | grep -c '^[[:space:]]*1' | wc -l
    // counting direct dependencies
    let mut deps_index = Command::new("cargo")
        .args(&["tree", "--prefix", "depth"])
        .current_dir(&analyzer_opts.CodePath)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run cargo command to calc direct deps");

    // we won't need deps_index anymore.
    let deps_index_output = deps_index.stdout.expect("Failed to run cargo command");

    let direct_deps = Command::new("grep")
        .args(&["-c", "^[[:space:]]*1"])
        .stdin(Stdio::from(deps_index_output))
        .stdout(Stdio::piped())
        .current_dir(&analyzer_opts.CodePath)
        .spawn()
        .expect("grepping failed to work");

    let mut direct_deps_output = direct_deps.stdout.expect("Failed again");
    // let mut total_deps_output = total_deps.stdout.expect("Total deps stdout failed.");
    let mut direct_deps_op = String::new();
    // let mut total_deps_op = String::new();
    direct_deps_output
        .read_to_string(&mut direct_deps_op)
        .unwrap();
    // total_deps_output
    //     .read_to_string(&mut total_deps_op)
    //     .unwrap();
    println!("***********");
    // println!("Total dependencies = {}", &total_deps_op);
    println!("Number of direct deps = {}", &direct_deps_op);
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
