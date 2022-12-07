use std::collections::HashMap;
use regex::Regex;

#[derive(Debug)]
struct FileInfo {
    path: Vec<String>,
    size: u32
}

fn find_files(filename: &str) -> Vec<FileInfo> {
    let mut cwd: Vec<String> = Vec::new();
    let mut res: Vec<FileInfo> = Vec::new(); 
    let cd_re = Regex::new(r"^\$ cd (.*)$").unwrap();
    let file_re = Regex::new(r"^(\d+) (.*)$").unwrap();
    for line in super::utils::read_lines(filename) {
        if line == "$ cd /" {
            cwd.clear();
        } else if line == "$ cd .." {
            cwd.pop();
        } else if let Some(cap) = cd_re.captures(&line) {
            cwd.push(String::from(&cap[1]));
        } else if let Some(cap) = file_re.captures(&line) {
            res.push(FileInfo {
                path: cwd.clone(),
                size: cap[1].parse::<u32>().unwrap()
            });
        }
    }
    return res;
}

fn collect_dirs(files: Vec<FileInfo>) -> HashMap<String, u32> {
    let mut res: HashMap<String, u32> = HashMap::new();
    for file in files {
        let mut path = file.path.clone();
        while !path.is_empty() {
            *res.entry(path.join("/")).or_insert(0) += file.size;
            path.pop();            
        }
        *res.entry(String::from("")).or_insert(0) += file.size;        
    }
    return res;
}

pub fn star1(filename: &str) {
    let dirs = collect_dirs(find_files(filename));
    println!("Star 1: {}",
             dirs.into_values().filter(|x| x <= &100000).sum::<u32>());
}

pub fn star2(filename: &str) {
    let dirs = collect_dirs(find_files(filename));
    let needed = 30000000 - (70000000 - dirs[""]);
    println!("Star 2: {}",
             dirs.into_values().filter(|x| x > &needed).min().unwrap());
}
