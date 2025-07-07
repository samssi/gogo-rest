use crate::util::filesystem::{find_root, set_permissions};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};
use tokio::fs as async_fs;
use tokio::io::AsyncWriteExt;

pub struct Buf {
    pub tools_bin_dir: PathBuf,
    pub buf_executable_path: PathBuf,
}

impl Buf {
    async fn download_prost_crate(tools_bin_dir: &PathBuf) {
        let prost_crate_file = tools_bin_dir.join("prost-crate");

        #[cfg(target_os = "macos")]
        let prost_url = "https://github.com/dflemstr/prost-crate/releases/latest/download/prost-crate-Darwin-aarch64";

        #[cfg(target_os = "linux")]
        let prost_url = "https://github.com/dflemstr/prost-crate/releases/latest/download/prost-crate-Linux-x86_64";

        #[cfg(target_os = "windows")]
        let prost_url = "https://github.com/dflemstr/prost-crate/releases/latest/download/prost-crate-Windows-x86_64.exe";

        println!("Downloading prost-crate from: {prost_url}");

        let bytes = reqwest::get(prost_url)
            .await
            .expect("Failed to download prost-crate")
            .bytes()
            .await
            .expect("Failed to read prost-crate binary");

        async_fs::File::create(prost_crate_file)
            .await
            .expect("Failed to create prost-crate binary")
            .write_all(&bytes)
            .await
            .expect("Failed to write prost-crate binary");

        set_permissions(tools_bin_dir, 0o755)
        // #[cfg(unix)]
        // fs::set_permissions(tools_bin_dir, fs::Permissions::from_mode(0o755))
        //     .expect("Failed to set prost-crate as executable");
    }

    async fn download_buf() -> Buf {
        let cargo_root = find_root(Path::new("cargo.toml")).expect("Could not find cargo.toml");
        let tools_bin_dir = cargo_root.join(".tools/bin");
        let buf_executable_path = tools_bin_dir.join("buf");

        if !buf_executable_path.exists() {
            println!(
                "Buf binary not found. Downloading to {}",
                buf_executable_path.display()
            );

            #[cfg(target_os = "macos")]
            let url = "https://github.com/bufbuild/buf/releases/latest/download/buf-Darwin-arm64";

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

            #[cfg(unix)]
            {
                fs::set_permissions(&buf_executable_path, fs::Permissions::from_mode(0o755))
                    .expect("Failed to set executable permissions");
                println!("Set buf binary as executable");
            }
        } else {
            println!(
                "Using cached buf binary at {}",
                buf_executable_path.display()
            );
        }

        Self {
            buf_executable_path,
            tools_bin_dir,
        }
    }

    async fn download_extensions(tools_bin_dir: &PathBuf) {
        Self::download_prost_crate(tools_bin_dir).await;
    }

    pub async fn exec_buf(&self) -> ExitStatus {
        let buf_gen_root =
            find_root(Path::new("buf.gen.yaml")).expect("Could not find buf.gen.yaml");
        let monorepo_root =
            find_root(Path::new("docker-compose.yml")).expect("Could not find docker-compose.yml");

        let buf_yaml = buf_gen_root.join("buf.gen.yaml");
        let protos_dir = monorepo_root.join("protos");

        Command::new(&self.buf_executable_path)
            .args(["generate", "--template"])
            .arg(buf_yaml)
            .current_dir(&protos_dir)
            .status()
            .expect("Failed to run buf")
    }

    pub async fn new() -> Self {
        println!("Starting buf code generation process...");
        let buf = Self::download_buf().await;
        Self::download_extensions(&buf.tools_bin_dir).await;

        buf
    }
}
