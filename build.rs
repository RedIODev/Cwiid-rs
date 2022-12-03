
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
    let mut builder = cc::Build::new();
    let build = builder
            .files(src.iter())
            .include("include")
            .flag("-Wno-unused-parameter");
    build.compile("cwiid");
}