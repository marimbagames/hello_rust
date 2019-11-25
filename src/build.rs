use prost_build;

fn main() {
    prost_build::compile_protos(&["src/hello.proto"], &["src/"]).unwrap();
}