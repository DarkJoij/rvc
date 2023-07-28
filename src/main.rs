mod macros;

use rvc_core::input::args::get_args;

fn main() {
    let args = get_args();
    println!("{args:#?}");
}
