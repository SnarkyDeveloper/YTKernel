fn main() {
    let var_name = "INVID_INSTANCE_URL";

    std::env::var(var_name).unwrap_or_else(|_| {
        panic!("Build Aborted: The environment variable '{}' is not set. Please set it before compiling.", var_name);
    });

    println!("cargo:rerun-if-env-changed={}", var_name);
}

