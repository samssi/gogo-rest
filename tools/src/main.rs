use crate::proto_gen::buf::Buf;
use crate::proto_gen::protoc::Protoc;

mod proto_gen;
mod util;

pub async fn run_protoc() {
    let protoc = Protoc::new().await;
    let protoc_exec_status = protoc.exec_protoc();

    if protoc_exec_status.success() {
        println!("protoc exited with success");
    } else {
        println!("protoc exited with error");
    }
}

pub async fn run_buf() {
    let buf = Buf::new().await;
    let buf_exec_status = buf.exec_buf().await;

    if buf_exec_status.success() {
        println!("Buf code generation completed successfully");
    } else {
        panic!("buf failed to run");
    }
}

#[tokio::main]
async fn main() {
    // TODO: Run buf breaking for breaking changes
    run_buf().await;
    //run_protoc().await;
}
