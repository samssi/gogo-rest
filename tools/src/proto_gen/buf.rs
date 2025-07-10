use crate::util::filesystem::{find_root, set_permissions};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};
use tokio::fs as async_fs;
use tokio::io::AsyncWriteExt;

pub struct Buf {
    pub monorepo_root_dir: PathBuf,
    pub tools_bin_dir: PathBuf,
    pub buf_executable_path: PathBuf,
}

impl Buf {
    pub fn download_protoc_gen_prost_crate(monorepo_root_dir: &PathBuf) {
        let status = Command::new("cargo")
            .args(["install", "protoc-gen-prost-crate", "--root", ".tools"])
            .current_dir(monorepo_root_dir)
            .status()
            .expect("Failed to run cargo install for protoc-gen-prost-crate");

        assert!(status.success(), "cargo install failed");
    }

    async fn download_buf() -> Buf {
        let monorepo_root =
            find_root(Path::new(".monoreporoot")).expect("Could not find .monoreporoot");
        let tools_bin_dir = monorepo_root.join(".tools/bin");
        let buf_executable_path = tools_bin_dir.join("buf");

        if !buf_executable_path.exists() {
            println!(
                "Buf binary not found. Downloading to {}",
                buf_executable_path.display()
            );

            #[cfg(target_os = "macos")]
            let url = "https://github.com/bufbuild/buf/releases/latest/download/buf-Darwin-arm64";
            println!("macos buf");

            #[cfg(target_os = "linux")]
            let url = "https://github.com/bufbuild/buf/releases/latest/download/buf-Linux-x86_64";

            #[cfg(target_os = "windows")]
            let url =
                "https://github.com/bufbuild/buf/releases/latest/download/buf-Windows-x86_64.exe";

            println!("Downloading buf from: {url}");
            fs::create_dir_all(&tools_bin_dir).expect("Failed to create bin directory");

            let bytes = reqwest::get(url)
                .await
                .expect("Failed to download buf")
                .bytes()
                .await
                .expect("Failed to read bytes from response");

            println!("Writing buf binary to {}", buf_executable_path.display());

            async_fs::File::create(&buf_executable_path)
                .await
                .expect("Failed to create buf binary")
                .write_all(&bytes)
                .await
                .expect("Failed to write buf binary");

            set_permissions(&buf_executable_path, 0o755)
        } else {
            println!(
                "Using cached buf binary at {}",
                buf_executable_path.display()
            );
        }

        Self {
            monorepo_root_dir: monorepo_root,
            buf_executable_path,
            tools_bin_dir,
        }
    }

    fn download_extensions(monorepo_root_dir: &PathBuf) {
        Self::download_protoc_gen_prost_crate(monorepo_root_dir);
    }

    pub async fn exec_buf(&self) -> ExitStatus {
        let monorepo_root =
            find_root(Path::new(".monoreporoot")).expect("Could not find .monoreporoot");
        let buf_gen_root = monorepo_root.join("gogo-rust-rest-app");
        let buf_yaml = buf_gen_root.join("buf.gen.yaml");
        let protos_dir = monorepo_root.join("protos");

        println!("Running buf generate");
        println!("Buf executable:   {}", self.buf_executable_path.display());
        println!("Template:     {}", buf_yaml.display());
        println!("Protos dir:   {}", protos_dir.display());
        println!("Tools dir:   {}", self.tools_bin_dir.display());

        let args = ["generate", "--template", buf_yaml.to_str().unwrap()];

        println!(
            "Running command: {} {}",
            self.buf_executable_path.display(),
            args.join(" ")
        );

        let status = Command::new(&self.buf_executable_path)
            .args(args)
            .env(
                "PATH",
                format!(
                    "{}:{}",
                    &self.tools_bin_dir.canonicalize().unwrap().display(),
                    std::env::var("PATH").unwrap()
                ),
            )
            .env("BUF_DEBUG", "1")
            .current_dir(&monorepo_root)
            .status()
            .expect("Failed to run buf");

        status
    }

    pub async fn new() -> Self {
        println!("Starting buf code generation process...");
        let buf = Self::download_buf().await;
        Self::download_extensions(&buf.monorepo_root_dir);
        buf
    }
}
