fn main() {
  tauri_build::build();
  let _ = tonic_build::compile_protos("./src/net/grpc/protos/nest_rpc.proto");
}
