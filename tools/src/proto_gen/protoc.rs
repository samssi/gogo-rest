use crate::util::filesystem::{find_root, set_permissions};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};
use tokio::fs as async_fs;
use tokio::io::AsyncWriteExt;

pub struct Protoc {
    pub tools_bin_dir: PathBuf,
    pub protoc_path: PathBuf,
    pub protoc_gen_prost_path: PathBuf,
    pub protoc_gen_tonic_path: PathBuf,
    pub protoc_gen_prost_crate_path: PathBuf,
}

impl Protoc {
    async fn download_plugin(tools_bin_dir: &PathBuf, name: &str, url: &str) -> PathBuf {
        let plugin_path = tools_bin_dir.join(name);

        if !plugin_path.exists() {
            println!("Downloading {} from: {}", name, url);
            fs::create_dir_all(&tools_bin_dir).expect("Failed to create bin directory");

            let bytes = reqwest::get(url)
                .await
                .expect("Failed to download plugin")
                .bytes()
                .await
                .expect("Failed to read plugin binary");

            async_fs::File::create(&plugin_path)
                .await
                .expect("Failed to create plugin binary")
                .write_all(&bytes)
                .await
                .expect("Failed to write plugin binary");

            set_permissions(&plugin_path, 0o755);
        }

        plugin_path
    }

    async fn download_protoc(tools_bin_dir: &PathBuf) -> PathBuf {
        let protoc_path = tools_bin_dir.join("protoc");

        if !protoc_path.exists() {
            println!("Downloading protoc...");

            #[cfg(target_os = "macos")]
            let url = "https://github.com/protocolbuffers/protobuf/releases/latest/download/protoc-25.3-osx-aarch_64.zip";

            #[cfg(target_os = "linux")]
            let url = "https://github.com/protocolbuffers/protobuf/releases/latest/download/protoc-25.3-linux-x86_64.zip";

            #[cfg(target_os = "windows")]
            let url = "https://github.com/protocolbuffers/protobuf/releases/latest/download/protoc-25.3-win64.zip";

            let zip_path = tools_bin_dir.join("protoc.zip");

            let bytes = reqwest::get(url)
                .await
                .expect("Failed to download protoc zip")
                .bytes()
                .await
                .expect("Failed to read protoc zip");

            async_fs::File::create(&zip_path)
                .await
                .expect("Failed to write protoc zip")
                .write_all(&bytes)
                .await
                .expect("Failed to write protoc zip");

            // Unzip manually
            let mut archive =
                zip::ZipArchive::new(std::fs::File::open(&zip_path).unwrap()).unwrap();
            for i in 0..archive.len() {
                let mut file = archive.by_index(i).unwrap();
                if file.name().ends_with("bin/protoc") || file.name().ends_with("bin/protoc.exe") {
                    let mut out = fs::File::create(&protoc_path).unwrap();
                    std::io::copy(&mut file, &mut out).unwrap();
                    set_permissions(&protoc_path, 0o755);
                    break;
                }
            }

            println!("Protoc installed to {}", protoc_path.display());
        } else {
            println!("Using cached protoc at {}", protoc_path.display());
        }

        protoc_path
    }

    pub async fn new() -> Self {
        let cargo_root = find_root(Path::new("cargo.toml")).expect("Could not find cargo.toml");
        let tools_bin_dir = cargo_root.join(".tools/bin");

        #[cfg(target_os = "macos")]
        let (prost_url, tonic_url, prost_crate_url) = (
            "https://github.com/someorg/protoc-gen-prost/releases/latest/download/protoc-gen-prost-macos",
            "https://github.com/someorg/protoc-gen-tonic/releases/latest/download/protoc-gen-tonic-macos",
            "https://github.com/dflemstr/prost-crate/releases/latest/download/prost-crate-Darwin-aarch64",
        );

        #[cfg(target_os = "linux")]
        let (prost_url, tonic_url, prost_crate_url) = (
            "https://github.com/someorg/protoc-gen-prost/releases/latest/download/protoc-gen-prost-linux",
            "https://github.com/someorg/protoc-gen-tonic/releases/latest/download/protoc-gen-tonic-linux",
            "https://github.com/dflemstr/prost-crate/releases/latest/download/prost-crate-Linux-x86_64",
        );

        let protoc_path = Self::download_protoc(&tools_bin_dir).await;
        let protoc_gen_prost_path =
            Self::download_plugin(&tools_bin_dir, "protoc-gen-prost", prost_url).await;
        let protoc_gen_tonic_path =
            Self::download_plugin(&tools_bin_dir, "protoc-gen-tonic", tonic_url).await;
        let protoc_gen_prost_crate_path =
            Self::download_plugin(&tools_bin_dir, "prost-crate", prost_crate_url).await;

        Self {
            tools_bin_dir,
            protoc_path,
            protoc_gen_prost_path,
            protoc_gen_tonic_path,
            protoc_gen_prost_crate_path,
        }
    }

    pub fn exec_protoc(&self) -> ExitStatus {
        let monorepo_root =
            find_root(Path::new("docker-compose.yml")).expect("Could not find docker-compose.yml");
        let protos_dir = monorepo_root.join("protos");
        let output_dir = monorepo_root.join("gogo-rust-rest-app/src/generated");
        let proto_file = "gogo/message/v1/message.proto";

        fs::create_dir_all(&output_dir).expect("Failed to create output dir");

        let command_string = format!(
            "protoc --proto_path={} --prost_out={} --tonic_out={} --prost-crate_out={} {}",
            protos_dir.display(),
            output_dir.display(),
            output_dir.display(),
            output_dir.display(),
            proto_file
        );

        println!("Running: {}", command_string);

        Command::new("protoc")
            .arg(format!("--proto_path={}", protos_dir.display()))
            .arg(format!("--prost_out={}", output_dir.display()))
            .arg(format!("--tonic_out={}", output_dir.display()))
            .arg(format!("--prost-crate_out={}", output_dir.display()))
            .arg("gogo/message/v1/message.proto")
            .status()
            .expect("Failed to run protoc")
    }
}
