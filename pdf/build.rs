use std::env;

fn main() {
    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    println!("cargo:rustc-link-search=native={}/lib/", project_dir); // the "-L" flag
    println!("cargo:rustc-link-lib=static=wkhtmltox"); // the "-l" flag    
}

