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

use std::io::{self, Write};
use std::process::Command;

struct AnalyzerOpts {
    Name: String,
    AnalysisConfigPath: String,
    FileFinderRegex: String,
    CodePath: String,
    ResultPath: String,
}

fn main() {
    let analyzer_opts = AnalyzerOpts {
        Name: String::from("javascript"),
        AnalysisConfigPath: String::from("/toolbox/analysis_config.json"),
        FileFinderRegex: String::from("\\.js$"),
        CodePath: String::from("/Users/sidntrivedi012/Code/cap"),
        ResultPath: String::from("/toolbox/analysis_results.json0"),
    };

    let output = Command::new("rustup")
        .args(&["update"])
        .current_dir(&analyzer_opts.CodePath)
        .output()
        .expect("ls command failed to start");

    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    assert!(output.status.success());

    let output = Command::new("rustup")
        .args(&["component", "add", "clippy"])
        .current_dir(&analyzer_opts.CodePath)
        .output()
        .expect("ls command failed to start");

    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    let output = Command::new("cargo")
        .args(&["clippy", "--", "-W", "clippy::all"])
        .current_dir(&analyzer_opts.CodePath)
        .output()
        .expect("ls command failed to start");

    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}
