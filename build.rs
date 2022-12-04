use std::env;


fn main() {
    // pkg_config::Config::new()
    //     .atleast_version("0.6.00")
    //     .probe("cwiid")
    //     .unwrap();
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
    let cc = env::var("CC_armv5te_unknown_linux_gnueabi").unwrap();
    cc::Build::new()
        .compiler(cc)
        .target("armv5te-linux-gnueabi")
        .files(src.iter())
        .include("include")
        .flag("-Wno-unused-parameter")
        .compile("cwiid");
}