
fn main() {
    pkg_config::Config::new()
        .atleast_version("0.6.00")
        .probe("cwiid")
        .unwrap();
    let src = [
        "include/bluetooth.c",
        "include/command.c",
        "include/connect.c",
        "include/interface.c",
        "include/process.c",
        "include/state.c",
        "include/thread.c",
        "include/util.c"
    ];
    let mut builder = cc::Build::new();
    let build = builder
            .files(src.iter())
            .include("include")
            .flag("-Wno-unused-parameter");
    build.compile("cwiid");
}