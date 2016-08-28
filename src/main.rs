#[macro_use]
extern crate quick_error;
extern crate toml;
extern crate clap;

use std::process::exit;

use toml::Table;
use clap::{App, ArgMatches, SubCommand};

mod config;
mod error;
mod cmd;
mod git;
mod cargo;

fn execute(args: &ArgMatches) -> Result<i32, error::FatalError> {
    let properties: Table = try!(config::parse_config());

    // Verify the TOML configuration.
    let valid_keys = vec![config::SIGN_COMMIT,
                          config::PUSH_REMOTE,
                          config::DOC_BRANCH,
                          config::DOC_COMMIT_MESSAGE];

    for key in properties.keys() {
        if !valid_keys.contains(&key.as_ref()) {
            println!("Unknown config key \"{}\" found for [package.metadata.gh-pages]",
                     key);
            return Ok(109);
        }
    }

    // Find parameters or use defaults.
    let dry_run = args.is_present("dry-run");
    let sign = args.is_present("sign") ||
               config::get_bool(&properties, config::SIGN_COMMIT).unwrap_or(false);
    let git_remote = args.value_of("push-remote")
        .or_else(|| config::get_str(&properties, config::PUSH_REMOTE))
        .unwrap_or("origin");
    let doc_branch = args.value_of("doc-branch")
        .or_else(|| config::get_str(&properties, config::DOC_BRANCH))
        .unwrap_or("gh-pages");
    let doc_commit_msg = args.value_of("doc-commit-message")
        .or_else(|| config::get_str(&properties, config::DOC_COMMIT_MESSAGE))
        .unwrap_or("(cargo-gh-pages) Generate docs.");

    // Check if working directory is clean.
    if !try!(git::status()) {
        println!("Uncommitted changes detected, please commit before release");
        if !dry_run {
            return Ok(101);
        }
    }

    // Generate and upload documentation.
    println!("Building and exporting docs.");
    try!(cargo::doc(dry_run));

    let doc_path = "target/doc/";

    try!(git::init(doc_path, dry_run));
    try!(git::add_all(doc_path, dry_run));
    try!(git::commit_all(doc_path, doc_commit_msg, sign, dry_run));
    let remote = try!(git::remote_get_url(git_remote));

    let mut refspec = String::from("master:");
    refspec.push_str(doc_branch);

    try!(git::force_push(doc_path, remote.trim(), &refspec, dry_run));

    Ok(0)
}


fn main() {
    let matches = App::new("cargo")
        .subcommand(SubCommand::with_name("gh-pages")
            .version(env!("CARGO_PKG_VERSION"))
            .author("Ning Sun <sunng@about.me>")
            .author("Woof Woof, Inc.")
            .about("Cargo subcommand for generating and publishing RustDoc to GitHub Pages.")
            .arg_from_usage("--dry-run 'Print commands to execute instead of running'")
            .arg_from_usage("--sign 'Sign git commit'")
            .arg_from_usage("--doc-commit-message=[doc-branch] 'Git commit message to use'")
            .arg_from_usage("--push-remote=[push-remote] 'Git remote for push'")
            .arg_from_usage("--doc-branch=[doc-branch] 'Git branch to push documentation on'"))
        .get_matches();

    if let Some(release_matches) = matches.subcommand_matches("gh-pages") {
        match execute(release_matches) {
            Ok(code) => exit(code),
            Err(e) => {
                println!("Fatal: {}", e);
                exit(128);
            }
        }
    }
}
