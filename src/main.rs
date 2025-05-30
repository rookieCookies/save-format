use save_format::parse_str;

pub fn main() { 
    let str = include_str!("../text_format.sft");
    let parsed = parse_str(str);
    dbg!(&parsed);

    println!("{}", save_format::to_string(parsed.unwrap()));

}
