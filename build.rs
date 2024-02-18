

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("building here");
    tonic_build::compile_protos("proto/ping.proto")?;
    Ok(())
}
