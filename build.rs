use cc;

fn main() {
    let mut config = cc::Build::new();

    let path = "external/lib/Remotery.c";

    config.file(path);

    config.compile("libremotery.a");
}
