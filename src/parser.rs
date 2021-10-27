
use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while};
use nom::character::complete::{alpha1, alphanumeric1, digit0, digit1, line_ending, multispace1, newline};
use nom::combinator::{cond, consumed, map, map_opt, opt, peek, recognize, value};
use nom::error::ParseError;
use nom::number::complete::{double, float};
use nom::sequence::{delimited, pair, separated_pair};

use crate::*;

use crate::Ftdtype::{ Integer,Decimal };




pub fn parser_cond(b: bool, i: &str) -> IResult<&str, Option<&str>> {
    cond(b, alpha1)(i)
}

pub fn uint(input:&str) -> IResult<&str,&str>{

        alt((
            recognize(pair(
                opt(tag("-")),
                digit1,
            )),

            tag("0")

        ))
        (input)
}

fn float_par(input:&str) -> IResult<&str,Ftdtype>{
    let parser = recognize(
        separated_pair(digit0,tag("."),digit1)
    );

    map(parser,|s : &str|{
        let n = s.parse::<f32>().unwrap();
        Ftdtype::Decimal(n)

    })(input)
}

fn digit_par(input:&str) -> IResult<&str,Ftdtype>{
    let parser = recognize(
       digit1 
    );

    map(parser,|s : &str|{
        let n = s.parse::<i32>().unwrap();
        Ftdtype::Integer(n)

    })(input)
}


fn inner_parser(input: &str) -> IResult<&str, bool> {
    value(false, tag("type"))(input)
}

pub fn ftd_parser(input:&'static str) -> IResult<&str,Variable>{
    let  mut parser = delimited(tag("--"), varparser, tag("--"));

    parser(input)
}

pub fn varparser(input:&'static str) -> IResult<&'static str,Variable>{


    let (input,_) = multispace1(input)?;
    let (input,_) = tag("var")(input)?;
    let (input,_) = multispace1(input)?;
    let (input,x) = alphanumeric1(input)?;
    let (input,_) = multispace1(input)?;
    let (input,_) = tag(":")(input)?;
    let (input,_) = multispace1(input)?;

    println!("{:?}",input);

    let (input,value) = alt((float_par,digit_par))(input)?;

    println!("{:?},{:?}",input,value);
    
    let dummy = Variable { name: x.to_string(), value: value, type_name: "Integer".to_string() };

    Ok(("",dummy))
}

