use logfmt_rs::parse;

fn main() {
    let line = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "level=info msg=\"hello world\"".to_string());
    match parse(&line) {
        Ok(json) => println!("{json}"),
        Err(e) => eprintln!("error: {e}"),
    }
}
