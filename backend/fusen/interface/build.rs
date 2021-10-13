fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../../../api/peta/fusen/v1/fusen.proto")?;

    Ok(())
}
