fn main() {
    #[cfg(all(target_family = "unix", not(target_os = "macos")))]
    {
        println!("cargo:rustc-link-arg-bin=tracer=-Wl,-z,undefs")
    }

    #[cfg(target_os = "macos")]
    {
        println!("cargo::rustc-link-arg-bin=tracer=-undefined");
        println!("cargo::rustc-link-arg-bin=tracer=dynamic_lookup");
    }
}
