use std::error::Error;


fn main()->Result<(),Box<dyn Error>>{
    tonic_prost_build::configure().compile_protos(
        &["schema/grpc/data.proto"],
        &["schema/grpc"]
    ).unwrap();

    capnpc::CompilerCommand::new()
        .src_prefix("schema/capnp")
        .file("schema/capnp/data.capnp")
        .run().unwrap();

    Ok(())
}
