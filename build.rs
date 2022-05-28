use cc;

fn main() {
    let mut compiler = cc::Build::new();

    let path = "external/lib/Remotery.c";

    compiler.file(path);

    compiler.opt_level(get_opt_level());

    compiler.compile("libremotery.a");
}

// "release" (`3`) by default, unless "debug" (`0`) is specified.
fn get_opt_level() -> u32 {
    std::env::var("PROFILE").map(|profile| {
        match profile.as_str() {
            "debug" => 0,
            "release" => 3,
            _ => 3,
        }
    }).unwrap_or(3)
}