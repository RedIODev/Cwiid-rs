use std::env;


fn main() {
    // pkg_config::Config::new()
    //     .atleast_version("0.6.00")
    //     .probe("cwiid")
    //     .unwrap();

    println!("cargo:rustc-link-lib=bluetooth");
    
    let src = [
        "src/bluetooth.c",
        "src/command.c",
        "src/connect.c",
        "src/interface.c",
        "src/process.c",
        "src/state.c",
        "src/thread.c",
        "src/util.c"
    ];
    let mut builder = cc::Build::new();
    let cc = env::var("CC_armv5te_unknown_linux_gnueabi");
    let builder = if let Ok(cc) = cc {
        println!("Using arm-linux");
        builder.compiler(cc).target("armv5te-linux-gnueabi")
    } else {
        &mut builder
    };
    builder
        .files(src.iter())
        .include("include")
        //.flag("-lbluetooth")
        .flag("-Wno-unused-parameter")
        .compile("cwiid");
}