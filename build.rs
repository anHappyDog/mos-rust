use std::process::Command;
use std::{env, fs, path::PathBuf};
const CLFAGS: &str = "--std=gnu99 -EL -G 0 -mno-abicalls -fno-pic
    -ffreestanding -fno-stack-protector -fno-builtin -msoft-float
    -nostdlib -nostartfiles -nodefaultlibs -mno-shared
    -Wa,-xgot -Wall -mxgot -mno-fix-r4000 -march=4kc";

fn read_cfiles_from_dir(dir: &str) -> Vec<PathBuf> {
    fs::read_dir(dir)
        .expect("Failed to read mos's c user source files")
        .filter_map(|e| {
            let p = e.unwrap().path();
            let ext = p.extension().and_then(|s| s.to_str());
            if ext == Some("c") || ext == Some("S") || ext == Some("s") {
                Some(p)
            } else {
                None
            }
        })
        .collect()
}

fn build_img() {}

fn compile_cfiles_for_mips32() {
    env::set_var("CFLAGS", CLFAGS);
    let _mos_cdir = "./user/bin";
    let mos_clib = "./user/lib";
    let mos_clibname = "mos_user";
    let mos_lib_out_dir = PathBuf::from("target/user/lib");
    let mos_bin_out_dir = "./target/user/bin/";
    let mos_c_proc_file: Vec<_> = read_cfiles_from_dir(_mos_cdir);
    let mos_cfile: Vec<_> = read_cfiles_from_dir(mos_clib);
    let mut build = cc::Build::new();
    build
        .include("./user/include")
        .compiler("mipsel-linux-gnu-gcc")
        .try_flags_from_environment("CFLAGS")
        .unwrap()
        .files(mos_cfile)
        .out_dir(mos_lib_out_dir)
        .compile(mos_clibname);
    std::fs::create_dir_all(mos_bin_out_dir).unwrap();

    for cfile in mos_c_proc_file {
        println!("{:?}", cfile);
        let output_path = PathBuf::from(mos_bin_out_dir).join(
            PathBuf::from(cfile.clone())
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap(),
        );

        let status = Command::new("mipsel-linux-gnu-gcc")
            .arg("-o")
            .arg(output_path)
            .arg(cfile.clone().as_os_str())
            .arg("-T./user/user.lds")
            .arg("-L./target/user/lib")
            .arg("-lmos_user")
            .arg("-I./user/include")
            .args(env::var("CFLAGS").unwrap_or_default().split_whitespace()) // 添加环境变量中的 CFLAGS
            .status()
            .expect("Failed to execute gcc");

        if !status.success() {
            panic!("Compilation failed for file: {:#?}", &cfile);
        }
    }
    // build the fs proc
    let fs_output_path = PathBuf::from(mos_bin_out_dir).join(std::path::Path::new("fs"));
    let status = Command::new("mipsel-linux-gnu-gcc")
        .arg("-o")
        .arg(fs_output_path)
        .arg("./user/fs/fs.c")
        .arg("./user/fs/serv.c")
        .arg("./user/fs/ide.c")
        .arg("-T./user/user.lds")
        .arg("-L./target/user/lib")
        .arg("-lmos_user")
        .arg("-I./user/include")
        .args(env::var("CFLAGS").unwrap_or_default().split_whitespace()) // 添加环境变量中的 CFLAGS
        .status()
        .expect("Failed to execute gcc");

    if !status.success() {
        panic!("Compilation fs server proc failed");
    }
    build_img();
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let target = env::var("TARGET").unwrap();

    match target.as_str() {
        "mips32" => compile_cfiles_for_mips32(),
        _ => {}
    }
}
