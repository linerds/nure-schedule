fn main() {
    if std::env::var("PROFILE") == Ok("debug".into()) {
        // For proxy tests
        println!("cargo::rustc-env=ALL_PROXY=socks5://127.0.0.1:9050");
        println!("cargo::rustc-env=HTTP_PROXY=socks5://127.0.0.1:9050");
        println!("cargo::rustc-env=HTTPS_PROXY=socks5://127.0.0.1:9050");
    }
}
