//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

use std::{collections::HashMap, process::Command, time::SystemTime, env};

fn main() {
    // In tests7, we should set up an environment variable
    // called `TEST_FOO`. Print in the standard output to let
    // Cargo do it.
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs(); // This timestamp is used as the value for the environment variable `TEST_FOO`.
    
    // Set the environment variable `TEST_FOO` using the timestamp.
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);
    
    // In tests8, we should enable "pass" feature to make the
    // testcase return early. Fill in the command to tell
    // Cargo about that.
    // Enable the "pass" feature.
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
