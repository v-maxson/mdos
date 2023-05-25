//! Generates a bootable image file from compiled kernel.

fn main() {
    let mut out_dir = std::env::current_exe().unwrap();
    out_dir.pop();

    println!("{}", out_dir.display());

    let kernel = out_dir.join("kernel");

    // create a BIOS disk image
    let bios_path = out_dir.join("mdos.img");
    bootloader::BiosBoot::new(&kernel).create_disk_image(&bios_path).unwrap();
}
