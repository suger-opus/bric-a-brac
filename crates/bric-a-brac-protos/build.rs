fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure()
        .build_client(false)
        .build_server(false)
        .build_transport(false)
        .type_attribute(".", "#[allow(clippy::enum_variant_names)]")
        .compile_protos(&["protos/common.proto"], &["protos/"])?;

    tonic_prost_build::configure()
        .type_attribute(".", "#[allow(clippy::enum_variant_names)]")
        .compile_protos(
            &[
                "protos/common.proto",
                "protos/ai.proto",
                "protos/knowledge.proto",
            ],
            &["protos/"],
        )?;

    println!("cargo:rerun-if-changed=protos/common.proto");
    println!("cargo:rerun-if-changed=protos/ai.proto");
    println!("cargo:rerun-if-changed=protos/knowledge.proto");

    Ok(())
}
