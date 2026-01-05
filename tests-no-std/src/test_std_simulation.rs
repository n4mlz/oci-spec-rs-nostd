//! Tests simulating no_std behavior in std environment.

use oci_spec::runtime::Spec;

fn main() {
    println!("Testing serde(default) behavior...");
    
    println!("\n1. Testing empty JSON: {{}}");
    test_empty_json();
    
    println!("\n2. Testing minimal version");
    test_minimal_version();
    
    println!("\n3. Testing version with default (empty JSON)");
    test_version_only();
    
    println!("\n4. Testing version and process");
    test_version_and_process();
    
    println!("\n5. Testing with root");
    test_with_root();
    
    println!("\nAll tests completed!");
}

fn test_empty_json() {
    let json = r#"{}"#;
    match serde_json::from_str::<Spec>(json) {
        Ok(spec) => {
            println!("  Success! version = {:?}", spec.version());
            if spec.version().is_empty() {
                println!("  WARNING: version is empty string (serde(default) used String::default())");
            }
        }
        Err(e) => {
            println!("  Error: {}", e);
        }
    }
}

fn test_minimal_version() {
    let json = r#"{"ociVersion": "1.0.2-dev"}"#;
    match serde_json::from_str::<Spec>(json) {
        Ok(spec) => {
            println!("  Success! version = {:?}", spec.version());
        }
        Err(e) => {
            println!("  Error: {}", e);
        }
    }
}

fn test_version_only() {
    let json = r#"{}"#;
    match serde_json::from_str::<Spec>(json) {
        Ok(spec) => {
            println!("  Success! version = {:?}", spec.version());
            println!("  process = {:?}", spec.process().is_some());
            println!("  root = {:?}", spec.root().is_some());
            println!("  linux = {:?}", spec.linux().is_some());
            
            if spec.process().is_some() {
                println!("  WARNING: process is Some (Default::default() was called)");
            }
            if spec.linux().is_some() {
                println!("  WARNING: linux is Some (Default::default() was called)");
            }
        }
        Err(e) => {
            println!("  Error: {}", e);
        }
    }
}

fn test_version_and_process() {
    let json = r#"{
        "ociVersion": "1.0.2-dev",
        "process": {
            "user": {},
            "args": ["sh"],
            "cwd": "/"
        }
    }"#;
    match serde_json::from_str::<Spec>(json) {
        Ok(spec) => {
            println!("  Success! version = {:?}", spec.version());
            println!("  process.args = {:?}", spec.process().as_ref().map(|p| p.args()));
        }
        Err(e) => {
            println!("  Error: {}", e);
        }
    }
}

fn test_with_root() {
    let json = r#"{
        "ociVersion": "1.0.2-dev",
        "process": {
            "user": {},
            "args": ["sh"],
            "cwd": "/"
        },
        "root": {
            "path": "rootfs",
            "readonly": true
        }
    }"#;
    match serde_json::from_str::<Spec>(json) {
        Ok(spec) => {
            println!("  Success! version = {:?}", spec.version());
            println!("  root.path = {:?}", spec.root().as_ref().map(|r| r.path()));
        }
        Err(e) => {
            println!("  Error: {}", e);
        }
    }
}
