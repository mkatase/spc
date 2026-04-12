// src/utils/common.rs

use std::collections::HashMap;
use crate::{CommandMap, FileConfig};

pub fn simplify_config(m: &CommandMap) -> HashMap<String, String> {
    let mut config = HashMap::new();
    let target_tags = ["FN", "FT", "FW", "FH", "BG"];

    for tag in target_tags {
        if let Some(val) = m.get(tag)
            .and_then(|v| v.first()) // first row
            .and_then(|r| r.get(1))  // first value
        {
            config.insert(tag.to_string(), val.clone());
        }
    }
    config
}
pub fn get_file_opt(k: &str, vv: &CommandMap) -> FileConfig {
    if let Some(lines) = vv.get(k) {
        if let Some(data) = lines.first() {
            return FileConfig {
                // data[0] is "FI"
                width:     data.get(1).and_then(|s| s.parse().ok()).unwrap_or(200.0),
                height:    data.get(2).and_then(|s| s.parse().ok()).unwrap_or(200.0),
                extension: data.get(3).cloned().unwrap_or_else(|| "webp".to_string()),
                name:      data.get(4).cloned().unwrap_or_else(|| "spc".to_string()),
            };
        }
    }
    FileConfig::default() 
}

pub fn make_map(vv: Vec<Vec<String>>) -> CommandMap {
    let mut m: CommandMap = CommandMap::new();

    for v in vv {
        m.entry(v[0].clone()).or_insert(Vec::new()).push(v);
    }
    //println!("{:?}",m);
    m
}
