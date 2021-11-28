#[macro_use]
extern crate log;

use simple_logger::SimpleLogger;
use std::io::BufRead;
use std::io::BufReader;
use std::process::Stdio;
use std::process::Command;
use std::io::ErrorKind;
use std::io::Write;

#[derive(Debug)]
enum Error {
    MissingArgEc,
    CannotOpenTest(std::io::Error),
    CannotStartLv6(std::io::Error),
    Lv6Error(std::process::ExitStatus),
}

fn main_result() -> Result<(), Error> {
    SimpleLogger::new().init().unwrap();

    let mut args = std::env::args().fuse();
    let _ = args.next();
    let ec = args.next().ok_or(Error::MissingArgEc)?;
    let test = args.next().unwrap_or_else(|| {
        warn!("No test file given, reading from stdin");
        String::from("/dev/stdin")
    });

    if !ec.ends_with(".ec") && !ec.ends_with(".lus") {
        warn!("Lustre input file doesn't end with .ec or .lus ({})", &ec);
    }

    let test_file = std::fs::read_to_string(test).map_err(Error::CannotOpenTest)?;
    let mut test_lines = test_file.lines().enumerate();
    let mut tests = Vec::new();
    let node_name = test_lines.next().filter(|(_, n)| !n.contains(" ")).expect("first line must be a node").1;
    for (idx, line) in test_lines {
        if line.starts_with('#') ||line.trim().is_empty() {
            continue;
        }

        let (inputs, outputs) = line.split_once("->").expect(&format!("no \"->\" found on line {}", idx));
        let inputs = inputs.split_whitespace().filter(|p| !p.is_empty()).collect::<Vec<&str>>();
        let outputs = outputs.split_whitespace().filter(|p| !p.is_empty()).collect::<Vec<&str>>();
        tests.push((inputs, outputs));
    }

    info!("Running lv6");

    let mut child = Command::new("lv6")
        .arg("-exec")
        .arg(ec)
        .arg("-n")
        .arg(node_name)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(Error::CannotStartLv6)?;
    
    let mut stdin = child.stdin.take().unwrap();
    let stdout = child.stdout.take().unwrap();

    for test in &tests {
        writeln!(&mut stdin, "{}", test.0.join(" ")).unwrap();
    }

    let total_tests = tests.len();
    let mut passing_tests = 0usize;
    let mut tests = tests.into_iter().peekable();
    for out_line in BufReader::new(stdout).lines() {
        let out_line = out_line.unwrap();
        if let Some((_, results)) = out_line.split_once("#outs") {
            let expected = tests.next().expect("lv6 outputed more steps than there were tests");
            let identical = results
                .split_whitespace()
                .zip(&expected.1)
                .all(|(a, b)| b.eq_ignore_ascii_case(a));

            if identical {
                passing_tests += 1;
                info!("Test passed {:?} -> {:?}", &expected.0, &expected.1);
            } else {
                info!("Test failed {:?} != {:?}", results, expected.1);
            }
        }

        if tests.peek().is_none() {
            break;
        }
    }

    let comment = match (passing_tests, total_tests) {
        (0, 0) => "All tests passed (none were defined)",
        (0, _) => "All tests failed",
        (a, b) if a == b => "All tests passed",
        (_, _) => "Some tests passed",
    };

    info!("{} ({}/{})", comment, passing_tests, total_tests);
    info!("Waiting for lv6 termination");
    std::mem::drop(stdin);
    let lv6_status = child.wait().unwrap();
    // TODO fix signal 13
    if false && !lv6_status.success() {
        Err(Error::Lv6Error(lv6_status))
    } else {
        Ok(())
    }
}

fn main() {
    if let Err(err) = main_result() {
        match err {
            Error::MissingArgEc => error!("usage: lustre-simple-test <circuit{{.ec|.lus}}> [test_file]"),
            Error::CannotOpenTest(err) if err.kind() == ErrorKind::NotFound => error!("Cannot open test file: {}", err),
            Error::CannotStartLv6(err) if err.kind() == ErrorKind::NotFound => error!("lv6 not found, is it installed ?"),
            Error::CannotStartLv6(err) => error!("Cannot start lv6: {}", err),
            Error::Lv6Error(status) => error!("lv6 exited with a non-null status code: {}", status),
            err => error!("{:?}", err),
        }
    }
}
