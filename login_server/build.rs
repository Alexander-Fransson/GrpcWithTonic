use std::error::Error;

use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    //let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    
    // enables reflection by generating a descriptor
    // compiles out protobuf into the file descriptor set
    tonic_build::configure()
    //.file_descriptor_set_path(out_dir.join("calculator_descriptor.bin"))
    .compile_protos(&["proto/login_server.proto"], &["proto/"])?;

    tonic_build::compile_protos("proto/login_server.proto")?;
    Ok(())
}