use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use regex::Regex;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file_name>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let lines = match read_lines(file_path) {
        Ok(lines) => lines,
        Err(_) => {
            eprintln!("Error: File '{}' not found.", file_path);
            std::process::exit(1);
        }
    };

    fs::remove_file(file_path)?;

    let filename = file_path
        .rsplit_once('.')
        .map(|(name, _)| name.replace("/", "-").to_uppercase())
        .unwrap_or_else(|| file_path.replace("/", "-").to_uppercase());

    let list_regex = vec![
        "voltage", "temperature0", "humidity0", "temperature1", "humidity1",
        "temperature2", "currents", "humidity2", "temperature3", "humidity3",
        "temperature4", "humidity4", "temperature5", "humidity5",
        "temperature6", "humidity6", "power", "pf", "energy", "frequency"
    ];

    let re_max_access = Regex::new(r"(MAX-ACCESS\s+read)(\s*$)").unwrap();
    let re_key_change = Regex::new(r"^.*KeyChange").unwrap();
    let re_current_obj = Regex::new(r"current OBJECT-TYPE").unwrap();
    let re_last_updated = Regex::new(r#"LAST-UPDATED.*""#).unwrap();
    let re_revision = Regex::new(r#"REVISION.*""#).unwrap();
    let re_definitions = Regex::new(r"^.*DEFINITIONS ::= BEGIN").unwrap();
    let re_sys_details = Regex::new(r"^.*\{ sysDetails.*").unwrap();

    let mut updated_lines = Vec::with_capacity(lines.len());
    for line in &lines {
        let mut updated_line = line.clone();
        updated_line = re_max_access.replace(&updated_line, "$1-only\n").to_string();
        updated_line = re_key_change.replace(&updated_line, "-- KeyChange").to_string();
        updated_line = re_current_obj.replace(&updated_line, "currents OBJECT-TYPE").to_string();
        updated_line = re_last_updated.replace(&updated_line, "LAST-UPDATED \"200510140000Z\"").to_string();
        updated_line = re_revision.replace(&updated_line, "REVISION \"200510140000Z\"").to_string();
        updated_lines.push(updated_line);
    }

    let precompiled_regexes: Vec<(Regex, String)> = list_regex.iter()
        .map(|pattern| {
            let regex = Regex::new(&format!(r"\b{}\b", pattern)).unwrap();
            (regex, pattern.to_string())
        })
        .collect();

    for i in 1..=32 {
        let mut new_lines = Vec::with_capacity(updated_lines.len());
        let mib_definition = format!("{}{} DEFINITIONS ::= BEGIN", filename, i);
        let index_line = format!("ippdu1 OBJECT IDENTIFIER ::=  {{ sysDetails {} }}", i);

        for line in &updated_lines {
            let mut new_line = re_definitions.replace(line, &mib_definition).to_string();
            new_line = re_sys_details.replace(&new_line, &index_line).to_string();

            if i > 1 {
                for (re, pattern) in &precompiled_regexes {
                    let replacement = format!("{}_{}", pattern, i - 1);
                    new_line = re.replace_all(&new_line, &replacement).to_string();
                }
            }

            new_lines.push(new_line);
        }

        let base_file = file_path.rsplit_once('.').map(|(f, _)| f).unwrap_or(file_path);
        let output_file = format!("{}_index_{}.mib", base_file, i);
        let file = File::create(output_file)?;
        let mut writer = BufWriter::new(file);

        for line in &new_lines {
            writeln!(writer, "{}", line.trim_end())?;
        }
    }

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

