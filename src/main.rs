use std::{env, fs, thread, time::Duration};

use time::OffsetDateTime;
use sevenz_rust;

enum DurationMode {
    PeriodicallySeconds(u64),
    Timestamp((u8, u8), u64),
}

fn main() -> ! {
    let encryption_key: String = env::var("ENCRYPTION_KEY").unwrap_or("".to_owned());
    if encryption_key.is_empty() {
        println!("!!NO ENCRYPTION_KEY WAS SET. PROGRAMM WILL COMPRESS WITHOUT ENCRYPTION!!");
    }

    let duration_mode: DurationMode = match env::var("TIME_UTC") {
        Ok(v) => {
            let skips: u64 = get_u64_from_env("SKIPS").unwrap_or(0);
            DurationMode::Timestamp(extract_time_from_string(&v), skips)
        }
        Err(_) => {
            let periodically_sec: u64 = get_u64_from_env("PERIODICALLY_SECONDS")
                .expect("TIME_UTC or PERIODICALLY_SECONDS have to be set");
            DurationMode::PeriodicallySeconds(periodically_sec)
        }
    };

    match duration_mode {
        DurationMode::PeriodicallySeconds(seconds) => {
            println!("Using perdiodically mode with {} seconds", seconds);
            let duration = Duration::from_secs(seconds);
            loop {
                backup(&encryption_key);
                thread::sleep(duration);
            }
        }
        DurationMode::Timestamp((hour, minute), skips) => {
            println!(
                "Using timestamp mode with hour {} and minute {} UTC",
                hour, minute
            );
            let duration = Duration::from_secs(60);
            let mut skips_count: u64 = 0;
            loop {
                let now = OffsetDateTime::now_utc();
                if hour == now.hour() && minute == now.minute() {
                    if skips <= skips_count {
                        backup(&encryption_key);
                        skips_count = 0;
                    } else {
                        skips_count += 1;
                    }
                }

                thread::sleep(duration);
            }
        }
    }
}

fn get_u64_from_env(key: &str) -> Option<u64> {
    let str: String = match std::env::var(key) {
        Ok(v) => v,
        Err(_) => return None,
    };

    let size: u64 = str
        .parse()
        .expect(&format!("Variable {} is not a number", key));

    return Some(size);
}

fn extract_time_from_string(time: &str) -> (u8, u8) {
    let time_splitted: Vec<&str> = time.split(":").collect();

    if time_splitted.len() != 2 {
        panic!("Invalid time {}. Format: HH:MM", time)
    }

    let hour_str: &str = time_splitted.get(0).unwrap();
    let hour: u8 = hour_str
        .parse()
        .expect(&format!("Not a valid hour {}", hour_str));
    let minute_str: &str = time_splitted.get(1).unwrap();
    let minute: u8 = minute_str
        .parse()
        .expect(&format!("Not a valid minute {}", minute_str));

    return (hour, minute);
}

fn compress_to_7z(src_dir: &str, dst_dir: &str, key: &str) {
    if let Err(e) = match key.is_empty() {
        true => sevenz_rust::compress_to_path(src_dir, dst_dir),
        false => sevenz_rust::compress_to_path_encrypted(src_dir, dst_dir, key.into()),
    } {
        eprintln!("{}", e);
    }
}

fn get_directories_to_backup() -> Vec<String> {
    let dirs = fs::read_dir("/volumes").unwrap();

    let paths: Vec<String> = dirs.map(|dir| {
        let dir = dir.unwrap();
        let path = dir.path();
        path.to_str().unwrap().to_owned()
    }).collect();

    return paths;
}

fn backup(key: &str) {
    let dirs: Vec<String> = get_directories_to_backup();
    if dirs.len() == 0 { 
        println!("Nothing to backup");
        return; 
    }
    println!("Starting Backup!");
    for dir in dirs {
        let dst_dir: String = dir.replace("/", "_");
        let dst_dir: String = format!("/archives/{}.7z", dst_dir);
        if key.is_empty() {
            println!("Compressing {} to {} WITHOUT ENCRYPTION", dir, dst_dir);
        } else {
            println!("Compressing {} to {} WITH ENCRYPTION", dir, dst_dir);
        }
        compress_to_7z(&dir, &dst_dir, key);
    }
    println!("Finished Backup!");
}