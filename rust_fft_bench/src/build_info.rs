pub fn print_report(name: &str) {
    println!("{{");
    println!("  \"name\": \"{name}\",");
    println!("  \"git_describe\": \"{}\",", env!("GIT_DESCRIBE"));
    println!("  \"git_hash\": \"{}\"", env!("GIT_HASH"));
    println!("}}");
}

pub fn git_describe() -> &'static str {
    env!("GIT_DESCRIBE")
}

pub fn git_hash() -> &'static str {
    env!("GIT_HASH")
}
