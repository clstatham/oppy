use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let kernel_name = env::var("CARGO_PKG_NAME")?;

    #[cfg(target_arch = "x86_64")]
    {
        println!("cargo:rustc-link-arg-bin={kernel_name}=--script=.cargo/linker.x86_64.ld");
        println!("cargo:rerun-if-changed=.cargo/linker.x86_64.ld");
    }

    println!("cargo:rustc-link-arg-bin={kernel_name}=--gc-sections");

    println!("cargo:rerun-if-env-changed=CARGO_PKG_NAME");
    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}
