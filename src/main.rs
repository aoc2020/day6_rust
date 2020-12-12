use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::{Regex, Match};
use std::fmt;
use std::collections::{HashMap, HashSet};

#[macro_use]
extern crate lazy_static;

const SHINY_GOLD: &str = "shiny gold";

fn anyCount(group: &Vec<String>) -> usize {
    let mut used_chars: HashSet<char> = HashSet::new();
    for s in group {
        for c in s.chars() {
            used_chars.insert(c);
        }
    }
    used_chars.len()
}

fn contains(s: &String, c: &char) -> bool {
    s.chars().into_iter().find(|i| i == c).is_some()
}

fn all(group: &Vec<String>, c: &char) -> bool {
    group.into_iter()
        .find(|s| !contains(s, c))
        .is_none()
}

fn allCount(group: &Vec<String>) -> usize {
    let init_chars = group[0].chars();
    init_chars.filter(|c| all(group, c)).count()
}

fn main() {
    let lines = read_file("input.txt");
    let groups = split_into_groups(lines);
    let any_counts: Vec<usize> = (&groups).into_iter().map(|g| anyCount(&g)).collect();
    let all_counts: Vec<usize> = (&groups).into_iter().map(|g| allCount(&g)).collect();
    let answer1: usize = any_counts.into_iter().sum();
    let answer2: usize = all_counts.into_iter().sum();
    println!("Answer 1: {}", answer1);
    println!("Answer 2: {}", answer2)
}

fn split_into_groups(lines: Vec<String>) -> Vec<Vec<String>> {
    let mut groups: Vec<Vec<String>> = vec!();
    let mut group: Vec<String> = vec!();
    for line in lines {
        if line == "" {
            groups.push(group);
            group = vec!();
        } else {
            group.push(line);
        }
    }
    groups.push(group);
    groups
}

fn read_file(filename: &str) -> Vec<String> {
    let file = File::open(filename)
        .expect(format!("File not found: {}", filename).as_str());
    let reader = BufReader::new(file);
    reader.lines().map(|s| s.expect("Could not collect line")).collect()
}