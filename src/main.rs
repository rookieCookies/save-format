use save_format::parse_str;
use sti::arena::Arena;

fn main() {
    dbg!(parse_str(&Arena::new(), include_str!("../text_format.sft")));
}
