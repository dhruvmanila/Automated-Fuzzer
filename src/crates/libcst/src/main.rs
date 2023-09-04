use std::env::args;

fn main() {
    let path = args().nth(1).unwrap().clone();

    let Ok(content) = std::fs::read_to_string(&path) else {
        return;
    };

    let _ = match libcst_native::tokenize(&content) {
        Ok(t) => t,
        Err(e) => {
            println!("{e}");
            return;
        }
    };
}
