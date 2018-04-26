#![feature(assoc_unix_epoch)]
#[macro_use]
extern crate nom;
extern crate rand;

use nom::IResult;
use std::error::Error;
use std::fs::File;
use std::io::Read;

#[cfg(test)]
mod tests;

pub mod flame_2d;

pub mod color;

pub struct Config {
    filename: String,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Too few arguments");
        }
        let filename = args[1].clone();
        Ok(Config { filename })
    }
}
pub fn run(config: Config) -> Result<(), Box<Error>> {
    // first up, .flame file parsing

    let mut f = File::open(config.filename)?;
    let mut bytes: Vec<u8> = Vec::new();
    let mut unprocessed;
    let count = f.read_to_end(&mut bytes)?;
    //read the 'flames' tag, if it exists.
    let res: IResult<&[u8], &[u8]> = flames_tag(&bytes);
    match res {
        IResult::Done(remaining, result) => {
            unprocessed = remaining;
            println!(
                "This group of flames is named: {}",
                String::from_utf8(result.to_vec())?
            )
        }
        IResult::Error(err) => {
            println!("Error reading \"flames\" tag: {:?}", err);
        }
        IResult::Incomplete(needed) => println!("Not enough characters were given..."),
    }
    Ok(())
}
named!(
    flames_tag,
    dbg!(ws!(delimited!(
        ws!(dbg!(tag!("<"))),
        ws!(take_until!(">")),
        ws!(tag!(">"))
    )))
);
