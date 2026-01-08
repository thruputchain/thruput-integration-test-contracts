use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <filename> [namespace]", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let namespace = if args.len() >= 3 {
        Some(&args[2])
    } else {
        None
    };

    let mut output = String::new();

    if let Some(ns) = namespace {
        output.push_str(&format!("namespace {} {{\n\n", ns));
    }

    // Manually define ChannelState XDR struct
    // This matches the Rust struct in interface.rs
    output.push_str(
        r#"struct ChannelState {
    uint64 channel_id;
    int128 balance_a;
    int128 balance_b;
    uint64 nonce;
};
"#,
    );

    if namespace.is_some() {
        output.push_str("\n\n} // namespace\n");
    }

    match fs::write(filename, output) {
        Ok(()) => println!("Successfully wrote types to {}", filename),
        Err(e) => {
            eprintln!("Error writing to file {}: {}", filename, e);
            std::process::exit(1);
        }
    }
}
