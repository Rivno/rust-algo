use ferris_says:: say;
use std::io::{stdout, BufWriter};

mod string_case;
mod vector_case;
mod linked_list_case;
mod integer_case;
mod tree_case;

fn main() {
    let stdout = stdout();
    let message = "Time for some leet-code fellow Rustaceans!";
    let width = message.chars().count();

    let writer = BufWriter::new(stdout.lock());
    say(message, width, writer).unwrap();
}
