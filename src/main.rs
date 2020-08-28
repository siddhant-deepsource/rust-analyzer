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

use serde_json;
use std::fs;
use std::io::{self, Write};
use std::process::Command;

struct AnalyzerOpts {
    Name: String,
    AnalysisConfigPath: String,
    FileFinderRegex: String,
    CodePath: String,
    ResultPath: String,
}

// struct Report {
//     reason :String,
//     code :{
//         code: String,
//     },
//     level : String,
//     message: String,
//     spans: [
//      line_end: i32,
//      line_start: i32
//     ]
// }

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

    io::stdout().write_all(&output.stdout).unwrap();
    println!("{}", String::from_utf8_lossy(&output.stdout));
    let my_json: Vec<serde_json::Value> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    fs::write("foo.txt", my_json).unwrap();
    // let my_json: Value = serde_json(String::from_utf8_lossy(&output.stdout)).unwrap()

    // println!("{}", my_json);
    io::stderr().write_all(&output.stderr).unwrap();
}
