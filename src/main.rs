use fs_extra::dir::{CopyOptions, copy as copy_dir};
use std::fs;
use std::path::{Path, PathBuf};

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
            options.copy_inside = true;
            copy_dir(path, target, &options)?;
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
    assert!(gecko_sdk.exists(), "the gecko-sdk folder is not installed");

    fs::create_dir_all(&mg24).unwrap();
    fs::create_dir_all(&cmsis).unwrap();
    fs::create_dir_all(&emlib).unwrap();

    let cmsis_folder = gecko_sdk.join("platform").join("CMSIS");
    copy_subfolders(&cmsis_folder, &cmsis).expect("Failed Copying CMSIS Files");
    println!("CMSIS copied to {}", cmsis.display());

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

    println!("Done now you can use the mg24-hal");
    println!(
        "WRANING: Please make sure you installed arm-none-eabi-gcc compiler and add it to the enviroment path"
    );
}
