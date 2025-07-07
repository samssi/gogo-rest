mod proto_gen;
mod util;
use crate::proto_gen::generator::Buf;

#[tokio::main]
async fn main() {
    let buf = Buf::new().await;
    let buf_exec_status = buf.exec_buf().await;

    if buf_exec_status.success() {
        println!("Buf code generation completed successfully");
    } else {
        panic!("buf failed to run");
    }
}
