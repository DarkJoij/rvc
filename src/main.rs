mod input;
mod macros;

use input::args::get_args;

fn main() {
    let args = get_args();
    println!("{args:#?}");
}
