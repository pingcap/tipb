// Copyright 2019 PingCAP, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fs;

fn main() {
    let out_dir = format!("{}/protos", std::env::var("OUT_DIR").unwrap());
    let mods: Vec<_> = fs::read_dir("proto")
            .expect("Couldn't read directory")
            .filter_map(|e| {
                let file_name = e.expect("Couldn't list file").file_name();
                let file_name = file_name.to_string_lossy();
                if !file_name.ends_with(".proto") {
                    return None;
                }
                Some(format!("proto/{}", file_name))
            })
            .collect();
    protobuf_build::generate_files(&["include".to_owned(), "proto".to_owned()], &mods, &out_dir);
}
