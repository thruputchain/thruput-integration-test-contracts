use groundhog_erc20_if::ERC20Error as Error;
use std::env;
use std::fs;
use xdrs::xdr_repr::XDR;

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

    output.push_str(&format!("{}", Error::def()));

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
