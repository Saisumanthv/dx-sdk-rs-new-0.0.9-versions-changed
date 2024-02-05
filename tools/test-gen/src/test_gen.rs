use std::fs::File;
use std::io::Write;
use std::{env, fs};

fn split_file_name(name: String, separator: &str) -> Vec<String> {
    let splitted_name = name.split(separator);
    let collection: Vec<&str> = splitted_name.collect();
    let mut converted_collection: Vec<String> = Vec::new();

    for item in collection {
        converted_collection.push(String::from(item));
    }

    converted_collection
}

fn read_dirs(path: &str) -> Vec<String> {
    let paths = fs::read_dir(path).unwrap();
    let mut names: Vec<String> = Vec::new();

    for dir in paths {
        let dir_abs_path = dir.unwrap().path().into_os_string().into_string().unwrap();
        let mut splitted_files_name: Vec<String> = split_file_name(dir_abs_path, "/");
        let files_name_with_extension = splitted_files_name.pop().unwrap();
        if files_name_with_extension.ends_with(".scen.json") {
            splitted_files_name = split_file_name(files_name_with_extension, ".");
            let files_names = String::from(splitted_files_name.first().unwrap());
            names.push(files_names);
        }
    }

    names.sort();
    names
}

fn print_denali_rs(file: &mut File, names: &[String]) {
    for name in names.iter() {
        writeln!(
            file,
            "#[test]
fn {}_rs() {{
    dharitri_wasm_debug::denali_rs(\"denali/{}.scen.json\", &contract_map());
}}
",
            name.replace('-', "_").to_lowercase(),
            name
        )
        .unwrap();
    }
}

fn print_denali_go(file: &mut File, names: &[String]) {
    for name in names.iter() {
        writeln!(
            file,
            "#[test]
fn {}_go() {{
    dharitri_wasm_debug::denali_go(\"denali/{}.scen.json\");
}}
",
            name.replace('-', "_").to_lowercase(),
            name
        )
        .unwrap();
    }
}

/// Example run:
/// `cargo run ../../contracts/examples/erc20/denali`
fn main() {
    let args: Vec<String> = env::args().collect();
    let files_path = &args[1];

    let names = read_dirs(files_path);

    let mut rs_file = File::create("denali_rs_test.rs").unwrap();
    print_denali_rs(&mut rs_file, names.as_slice());

    let mut go_file = File::create("denali_go_test.rs").unwrap();
    print_denali_go(&mut go_file, names.as_slice());
}
