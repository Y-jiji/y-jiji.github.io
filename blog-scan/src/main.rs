use std::str::FromStr;

// brainless modularization, since we don't have much stuff
mod summary_writer;
mod markdown_to_json;
mod filescan;
use std::collections::{HashMap, HashSet};
use markdown_to_json::parse_markdown_front_matter;
use serde::*;

// the scanned post folder
const POST: &str = "post";
// the output folder
const INFO: &str = "info";

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
struct TopicItem {
    abstr: Option<String>,
    title: Option<String>,
    ahref: Option<String>,
    tag: Vec<String>,
    date: Option<String>,
}

// a simple script for parsing markdown into json indices
fn main() {
    let mut dir = std::env::current_dir().unwrap()
        .to_str().unwrap().to_string();
    {
        let mut ap = argparse::ArgumentParser::new();
        ap.refer(&mut dir)
            .add_option(&["-D", "--dir"], argparse::Store, "");
        ap.parse_args_or_exit();
    }
    let dir = std::path::PathBuf::from_str(&dir).unwrap();
    println!("{dir:?}");
    let posts = filescan::scan(&dir);
    let mut meta_table: HashMap<String, serde_yaml::Value> = HashMap::new();
    // parse yaml frontmatters into a meta table
    for post in posts {
        let mdfi = std::fs::read_to_string(&post).unwrap();
        let json = parse_markdown_front_matter(mdfi)
            .expect(&format!("when working with {post:?}, we found yaml error"));
        let post = match post.strip_prefix(dir.join(POST)) {
            Ok(post) => post,
            Err(_) => &post,
        };
        let post = post.components()
            .map(|x| x.as_os_str())
            .fold(String::new(), |x, y| x + y.to_str().unwrap());
        meta_table.insert(post, json);
    }
    // parse yaml frontmatters into a meta table, dump metatable to json!
    let json_value = serde_json::to_value(meta_table.clone()).unwrap();
    std::fs::write(dir.join(INFO).join("table-meta.json"), json_value.to_string()).unwrap();
    // build full indices on meta table, put them to info folder as index-*.json
    let mut json_index = HashMap::<String, HashMap<serde_yaml::Value, HashSet<String>>>::new();
    // json for topic
    let mut json_topic = HashMap::<String, Vec<TopicItem>>::new();
    for (file, value) in meta_table.iter() {
        match value {
            serde_yaml::Value::Mapping(mapping) => {
                let mut has_topic = false;
                let mut has_title = false;
                let mut has_summa = false;
                let mut has_date  = false;
                let mut topic_entry = (None::<String>, TopicItem::default());
                topic_entry.1.ahref = Some(file.clone());
                for (key, value) in mapping.into_iter() {
                    let key = key.as_str().unwrap().to_string();
                    if &key == "topic" { has_topic = true; }
                    if &key == "title" { has_title = true; }
                    if &key == "abstr" { has_summa = true; }
                    if &key == "date"  { has_date  = true; }
                    if key == "topic" {
                        topic_entry.0 = value.as_str().map(|s| s.to_string());
                    } else if key == "abstr" {
                        topic_entry.1.abstr = value.as_str().map(|s| s.to_string());
                    } else if key == "title" {
                        topic_entry.1.title = value.as_str().map(|s| s.to_string());
                    } else if key == "date" {
                        topic_entry.1.date = value.as_str().map(|s| s.to_string());
                    } else if key == "tag" {
                        match value {
                            serde_yaml::Value::String(s) => topic_entry.1.tag.push(s.to_string()),
                            serde_yaml::Value::Sequence(s) => topic_entry.1.tag.extend(s.iter().filter_map(|s| s.as_str()).map(|s| s.to_string())),
                            _ => println!("Warning: unrecognized tag {value:?}"),
                        };
                    }
                    let mut for_each_entry = |key: &String, value: serde_yaml::Value, file: &String| {
                        json_index.entry(key.to_string())
                            .and_modify(|e| {e.entry(value.clone()).and_modify(|e| {e.insert(file.to_string());}).or_insert(HashSet::from([file.to_string()])); })
                            .or_insert(HashMap::from([(value, HashSet::from([file.to_string()]))]));
                    };
                    let process_string = |s: &String| {
                        s.split(|c: char| !(c.is_alphabetic() && c.is_alphanumeric()))
                            .filter(|x| x.len() > 0)
                            .map(|x| serde_yaml::Value::String(x.to_string()))
                            .chain([serde_yaml::Value::String(s.to_string())])
                            .collect::<Vec<_>>()
                    };
                    match value {
                        serde_yaml::Value::Sequence(seq) => {
                            for value in seq.into_iter().flat_map(
                                    |v| {match v { 
                                        serde_yaml::Value::String(s) => process_string(s),
                                        v => vec![v.to_owned()],
                                    }}
                                ) {
                                for_each_entry(&key, value, file);
                            }
                        },
                        value if !value.is_mapping() => {
                            let value = match value {
                                serde_yaml::Value::String(s) => process_string(s),
                                v => vec![v.to_owned()],
                            };
                            for value in value {
                                for_each_entry(&key, value, file);
                            }
                        }
                        _ => println!("Warning: because {value:?} is a mapping, it is not parsed!")
                    }
                }
                if !has_date {
                    println!("Warning: {file:?} doesn't have date label, so we will not put it on the home page!");
                }
                if !has_topic {
                    println!("Warning: {file:?} doesn't have topic label, so we will not put it on the home page!");
                } else {
                    json_topic.entry(topic_entry.0.unwrap())
                        .and_modify(|v| v.push(topic_entry.1.clone()))
                        .or_insert(vec![topic_entry.1]);
                }
                if !has_summa {
                    println!("Warning: {file:?} doesn't have abstr label, so we will not generate an summary ont the home page");
                }
                if !has_title {
                    println!("Warning: {file:?} doesn't have title label, so we will not put it as a blog post!")
                }
            },
            _ => println!("Warning: in {file:?}, because {value:?} is not a mapping, it is not parsed!")
        }
    }
    for (index_name, index_file) in json_index {
        std::fs::write(
            dir.join(INFO).join(format!("index-{index_name}.json")), 
            serde_json::to_value(index_file).unwrap().to_string()).unwrap();
    }
    std::fs::write(dir.join(INFO).join("topic-list.json"), serde_json::to_value(json_topic).unwrap().to_string()).unwrap();
}