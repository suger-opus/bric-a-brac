fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .type_attribute(".", "#[allow(clippy::enum_variant_names)]")
        .compile_protos(&["protos/knowledge.proto"], &["protos/"])?;

    println!("cargo:rerun-if-changed=protos/knowledge.proto");

    Ok(())
}
