use std::io::BufRead;
use std::io::BufReader;
use std::process::Stdio;
use std::process::Command;
use std::io::Write;

#[derive(Debug)]
enum Error {
    MissingArgEc,
}

fn main_result() -> Result<(), Error> {
    let mut args = std::env::args().fuse();
    let _ = args.next();
    let ec = args.next().ok_or(Error::MissingArgEc)?;
    let test = args.next().unwrap_or_else(|| {
        eprintln!("[warning] No test file given, reading from stdin");
        String::from("/dev/stdin")
    });

    if !ec.ends_with(".ec") && !ec.ends_with(".lus") {
        eprintln!("[warning] Lustre input file doesn't end with .ec or .lus ({})", &ec);
    }

    let test_file = std::fs::read_to_string(test).unwrap();
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

    eprintln!("[info] Running lv6");

    let mut child = Command::new("lv6")
        .arg("-exec")
        .arg(ec)
        .arg("-n")
        .arg(node_name)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    
    let mut stdin = child.stdin.take().unwrap();
    let stdout = child.stdout.take().unwrap();

    for test in &tests {
        writeln!(&mut stdin, "{}", test.0.join(" ")).unwrap();
    }

    let mut tests = tests.into_iter().peekable();
    for out_line in BufReader::new(stdout).lines() {
        let out_line = out_line.unwrap();
        if let Some((_, results)) = out_line.split_once("#outs") {
            let expected = tests.next().unwrap();
            let identical = results
                .split_whitespace()
                .zip(&expected.1)
                .all(|(a, b)| a == *b);

            if identical {
                eprintln!("[test OK] {:?} -> {:?}", &expected.0, &expected.1);
            } else {
                eprintln!("[test BAD] {:?} != {:?}", results, expected);
            }
        }

        if tests.peek().is_none() {
            break;
        }
    }

    std::mem::drop(stdin);
    child.wait().unwrap();

    Ok(())
}

fn main() {
    main_result().unwrap();
}
