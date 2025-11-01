use std::error::Error;


fn main()->Result<(),Box<dyn Error>>{
    tonic_prost_build::configure().compile_protos(
        &["proto/data.proto"],
        &["proto"]
    ).unwrap();

    capnpc::CompilerCommand::new()
        .src_prefix("capnp")
        .file("capnp/data.capnp")
        .run().expect("schema compiler command");

    Ok(())
}
