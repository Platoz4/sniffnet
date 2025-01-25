use crate::utils::formatted_strings::APP_VERSION;
use crate::SNIFFNET_LOWERCASE;
use std::collections::HashMap;
use std::net::IpAddr;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

pub fn fetch_blacklist(ip_blacklist: &Arc<Mutex<HashMap<IpAddr, usize>>>) {
    let client = reqwest::blocking::Client::new();
    let blacklist = client
        .get("https://raw.githubusercontent.com/stamparm/ipsum/master/ipsum.txt")
        .header("User-agent", format!("{SNIFFNET_LOWERCASE}-{APP_VERSION}"))
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .send()
        .unwrap()
        .text()
        .unwrap();

    for line in blacklist.lines() {
        if !line.trim().is_empty() && !line.contains('#') {
            let [ip_str, count_str]: [&str; 2] = line
                .split_whitespace()
                .collect::<Vec<&str>>()
                .try_into()
                .unwrap_or_default();
            let (Ok(ip), Ok(count)) = (IpAddr::from_str(ip_str), count_str.parse::<usize>()) else {
                continue;
            };
            ip_blacklist.lock().unwrap().insert(ip, count);
        }
    }
}
