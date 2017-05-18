#[macro_use]
extern crate clap;
extern crate mozlog;
extern crate serde;
extern crate serde_json;

use std::io::{BufReader, BufRead};
use std::fs::File;
use std::path::{PathBuf, Path};
use std::collections::BTreeMap;

use clap::{App, Arg};

fn app<'a, 'b>() -> App<'a, 'b> {
    App::new(format!("mozlog {}", crate_version!()))
        .about("Utility for working with mozlog files.")
        .arg(Arg::with_name("paths")
             .required(true)
             .multiple(true)
             .help("Path of log file to parse"))
}

fn main() {
    let matches = app().get_matches();
    let paths = matches.values_of("paths").unwrap().map(|x| PathBuf::from(x));

    let mut results: BTreeMap<String, (Vec<mozlog::Status>, BTreeMap<String, Vec<mozlog::SubStatus>>)> = BTreeMap::new();

    let mut buffer = String::new();
    for path in paths {
        println!("{:?}", path);
        if !Path::exists(&*path) {
            println!("{:?} doesn't exist", path);
            continue;
        }
        let mut reader = BufReader::new(File::open(path).unwrap());
        while reader.read_line(&mut buffer).unwrap() > 0 {
            let msg: mozlog::LogMessage = match serde_json::from_str(&*buffer) {
                Ok(x) => x,
                Err(e) => {
                    println!("{}\n{}", buffer, e);
                    panic!();
                }
            };
            match msg {
                mozlog::LogMessage::TestStart(x) => {
                    if !results.contains_key(&x.test) {
                        results.insert(x.test, (vec![], BTreeMap::new()));
                    }
                },
                mozlog::LogMessage::TestStatus(x) => {
                    if let Some(result) = results.get_mut(&x.test) {
                        if !result.1.contains_key(&x.subtest) {
                            result.1.insert(x.subtest.clone(), vec![]);
                        }
                        let sub_results = result.1.get_mut(&x.subtest).unwrap();
                        sub_results.push(x.status);
                    };
                },
                mozlog::LogMessage::TestEnd(x) => {
                    if let Some(result) = results.get_mut(&x.test) {
                        result.0.push(x.status);
                    }
                },
                _ => {}
            };
            buffer.clear();
        }
    }

    for (test, data) in results.iter() {
        let &(ref test_status, ref subtests) = data;
        if !test_status.iter().all(|x| *x == test_status[0]) {
            println!("{}, {:?}", test, test_status);
        }
        for (subtest, subtest_status) in subtests.iter() {
            if !subtest_status.iter().all(|x| *x == subtest_status[0]) {
                println!("{}, {}, {:?}", test, subtest, data);
            }
        };
    };
}
