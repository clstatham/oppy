use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Get the name of the package.
    let kernel_name = env::var("CARGO_PKG_NAME")?;

    // Add linker args.
    println!("cargo:rustc-link-arg-bin={kernel_name}=--script=.cargo/linker.ld"); // Use the linker script.
    println!("cargo:rustc-link-arg-bin={kernel_name}=--gc-sections"); // Remove unused sections.

    // Have cargo rerun this script if the linker script, CARGO_PKG_NAME, or build.rs changes.
    println!("cargo:rerun-if-changed=.cargo/linker.ld");
    println!("cargo:rerun-if-env-changed=CARGO_PKG_NAME");
    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}
