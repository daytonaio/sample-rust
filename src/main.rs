extern crate ferris_says;

use std::io::{ stdout, BufWriter };

fn main() {
    let out: &str = "Hello fellow Rustaceans!";

    let mut writer: BufWriter<std::io::Stdout> = BufWriter::new(stdout());
    ferris_says::say(out, out.len(), &mut writer).unwrap();
}