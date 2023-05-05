use std::{path::PathBuf, time::{SystemTime, UNIX_EPOCH}};
use walkdir::WalkDir;
use chrono::{DateTime, Utc, TimeZone};

fn to_datatime(t: SystemTime) -> DateTime<Utc> {
    let (sec, nsec) = match t.duration_since(UNIX_EPOCH) {
        Ok(dur) => (dur.as_secs() as i64, dur.subsec_nanos()),
        Err(e) => { // unlikely but should be handled
            let dur = e.duration();
            let (sec, nsec) = (dur.as_secs() as i64, dur.subsec_nanos());
            if nsec == 0 {
                (-sec, 0)
            } else {
                (-sec - 1, 1_000_000_000 - nsec)
            }
        },
    };
    Utc.timestamp_opt(sec, nsec).unwrap()
}

pub fn scan(curr_dir: &PathBuf) -> Vec<PathBuf> {
    let post_dir = curr_dir.join(crate::POST);
    let post_dir = WalkDir::new(post_dir).into_iter().filter_map(Result::ok);
    let post_dir = post_dir.filter(
        |x| x.file_name().to_str().unwrap().ends_with(".md")
    );
    let post_dir = post_dir.map(|x| x.into_path());
    return post_dir.collect();
}