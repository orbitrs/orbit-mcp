// Build script for orbit-mcp

use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // Check if we need to install any dependencies
    if cfg!(target_os = "linux") {
        // Check if required system libraries are installed
        let output = Command::new("pkg-config")
            .args(&["--exists", "openssl"])
            .output()
            .expect("Failed to execute pkg-config");
            
        if !output.status.success() {
            println!("cargo:warning=OpenSSL development libraries not found. You may need to install libssl-dev.");
        }
    }

    // Print some information about the build
    println!("cargo:warning=Building orbit-mcp MCP server...");
    
    if let Ok(profile) = env::var("PROFILE") {
        println!("cargo:warning=Build profile: {}", profile);
    }
    
    // Add any source files that need special handling
    
    // Notify about command-line options
    println!("cargo:warning=Use the --features option to enable additional features.");
    println!("cargo:warning=Available features: none currently");
}
