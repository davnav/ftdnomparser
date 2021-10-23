
use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, digit0, digit1, multispace1};
use nom::number::complete::float;

use crate::*;

use crate::Ftdtype::Integer;


pub fn varparser(input:&str) -> IResult<&str,Variable>{

    let (input,_) = tag("--")(input)?;

    let (input,_) = multispace1(input)?;
    let (input,_) = tag("var")(input)?;

    let (input,_) = multispace1(input)?;
    

    let (input,x) = alphanumeric1(input)?;

    let (input,_) = multispace1(input)?;
    let (input,_) = tag(":")(input)?;

    let (input,_) = multispace1(input)?;
    
    println!("{:?}",input);

    let (input,value) = digit1(input)? ;
    
    println!("{:?}",input);
    
    let dummy = Variable { name: x.to_string(), value: Integer(value.parse::<i32>().unwrap()), type_name: "Integer".to_string() };

    Ok(("",dummy))
}

