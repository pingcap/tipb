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

fn main() {
    if !cfg!(feature = "gen") {
        println!("cargo:rerun-if-changed=build.rs");
        return;
    }

    #[cfg(feature = "gen")]
    {
        use protobuf_build::*;
        use std::fs;

        check_protoc_version();

        let mut mods: Vec<_> = fs::read_dir("proto")
            .expect("Couldn't read directory")
            .filter_map(|e| {
                let file_name = e.expect("Couldn't list file").file_name();
                let file_name = file_name.to_string_lossy();
                if !file_name.ends_with(".proto") {
                    return None;
                }
                Some(file_name[..file_name.len() - 6].to_owned())
            })
            .collect();
        mods.sort();
        let protos: Vec<_> = mods.iter().map(|m| format!("proto/{}.proto", m)).collect();

        for p in &protos {
            println!("cargo:rerun-if-changed={}", p);
        }

        // Prost
        generate_prost_files(&protos, "src/prost");
        generate_wrappers(&["src/prost/tipb.rs"], "src/prost", GenOpt::MUT
            | GenOpt::TRIVIAL_GET
            | GenOpt::TRIVIAL_SET
            | GenOpt::HAS
            | GenOpt::TAKE
            | GenOpt::CLEAR,);
        fs::remove_file("src/prost/gogoproto.rs").unwrap();
        fs::remove_file("src/prost/google.protobuf.rs").unwrap();
    }
}
