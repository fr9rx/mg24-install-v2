use fs_extra::dir::{copy as copy_dir, CopyOptions};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn copy_subfolders(src: &Path, target: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if !src.exists() {
        return Err(format!("Source folder does not exist: {}", src.display()).into());
    }

    fs::create_dir_all(target)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let mut options = CopyOptions::new();
            options.overwrite = true;
            options.copy_inside = true;
            copy_dir(path, target, &options).expect("Cant Copy Directories");
        }
    }

    Ok(())
}

fn main() {
    let home_dir = dirs::home_dir().expect("cant find hoem directory");
    let gecko_sdk = home_dir.join("Downloads").join("gecko-sdk");
    let mg24 = home_dir.join(".mg24");
    let cmsis = home_dir.join(".cmsis");
    let emlib = home_dir.join(".emlib");
    let common = home_dir.join(".cmsis");
    assert!(gecko_sdk.exists(), "the gecko-sdk folder is not installed");

    fs::create_dir_all(&mg24).expect("cant create directory .mg24");
    fs::create_dir_all(&cmsis).expect("cant create directory .cmsis");
    fs::create_dir_all(&emlib).expect("cant create directory .emlib");

    let cmsis_folder = gecko_sdk.join("platform").join("CMSIS");
    copy_subfolders(&cmsis_folder, &cmsis).expect("Failed Copying CMSIS Files");
    println!("CMSIS copied to {}", cmsis.display());

    let common_folder = gecko_sdk.join("paltform").join("common");
    let mut options = CopyOptions::new();
    options.overwrite = true;
    options.copy_inside = false;
    copy_dir(common_folder, &common, &options).expect("Failed Copying Common Files");
    println!("Common Copied to {}", &common.display());

    let emlib_folder = gecko_sdk.join("platform").join("emlib");
    copy_subfolders(&emlib_folder, &emlib).expect("Failed Copying Emlib Files");
    println!("Emlib copied to {}", emlib.display());

    let device_folder = gecko_sdk
        .join("platform")
        .join("Device")
        .join("SiliconLabs")
        .join("EFR32MG24");
    copy_subfolders(&device_folder, &mg24).expect("Failed Copying Device Files");
    println!("Device Files copied to {}", mg24.display());

    let output = Command::new("rustup")
        .args(["target", "add", "thumbv8m.main-none-eabihf"])
        .output()
        .expect("failed to run rustup target add");

    if output.status.success() {
        println!("Target Installing done");
        println!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("Target failed to Installed");
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }

    let probe_rs_output = Command::new("cargo")
        .args(["install", "probe-rs-tools", "--locked"])
        .output()
        .expect("failed to run cargo install probe-rs-tools");

    if probe_rs_output.status.success() {
        println!("Probe-rs Installing done");
        println!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("Probe-rs Failed to Installed");
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }

    println!("Done now you can use the mg24-hal");
    println!(
        "WRANING: Please make sure you installed arm-none-eabi-gcc compiler and add it to the enviroment path"
    );
}
