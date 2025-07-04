mod mod_gen;

use crate::mod_gen::generate_mod_rs;
use fs::create_dir_all;
use std::env::current_dir;
use std::fs;
use std::path::PathBuf;

fn find_monorepo_root() -> Option<PathBuf> {
    let mut dir = current_dir().ok()?;

    while dir.parent().is_some() {
        if dir.join("protos").exists() {
            return Some(dir);
        }

        dir = dir.parent()?.to_path_buf();
    }

    None
}

fn main() {
    let monorepo_root = find_monorepo_root().expect("Monorepo root not found");

    let proto_root = monorepo_root.join("protos");
    let proto_files = vec![proto_root.join("gogo/message/v1/message.proto")];

    let out_dir = monorepo_root
        .join("gogo-rust-rest-app")
        .join("src")
        .join("generated");
    create_dir_all(&out_dir).expect("Failed to create output directory");

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir(&out_dir)
        .compile_protos(&proto_files, &[proto_root])
        .expect("Failed to compile protos");

    generate_mod_rs(&out_dir).expect("Failed to generate mod.rs");

    println!("Protobufs compiled to gogo-rust-rest-app/src/generated");
}
