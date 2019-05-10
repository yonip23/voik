mod commit_log;

extern crate dirs;

use commit_log::CommitLog;
use std::time::SystemTime;

use std::fs;

fn main() -> Result<(), std::io::Error> {
    let mut target_path = dirs::home_dir().unwrap();
    target_path.push("loglady/");

    fs::remove_dir_all(target_path.clone())?;
    println!("👵 loglady logging to {:?}", target_path);

    let segment_size = 1_048_576; // 1MB
    let total_messages = 100_000;
    let mut clog = CommitLog::new(target_path, segment_size)?;

    let now = SystemTime::now();
    for _ in 0..total_messages {
        clog.write(b"one day my log will have something to say about this|")?;
    }

    let ms = now.elapsed().unwrap().as_millis();
    println!("{} messages written in {} ms", total_messages, ms);

    Ok(())
}
