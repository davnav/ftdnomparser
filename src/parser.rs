
use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until, take_while};
use nom::character::complete::{alpha0, alpha1, alphanumeric1, crlf, digit0, digit1, line_ending, multispace0, multispace1, newline, space1};
use nom::combinator::{cond, consumed, map, map_opt, opt, peek, recognize, value};
use nom::error::ParseError;
use nom::number::complete::{double, float};
use nom::sequence::{delimited, pair, separated_pair};

use crate::*;

use crate::Ftdtype::{ Integer,Decimal };

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


pub fn ftd_parser(input:&'static str) -> IResult<&'static str,Variable>{
    let  mut parser = delimited(tag("--"), varparser, tag("--"));

    parser(input)
}

pub fn varparser(input:&'static str) -> IResult<&str,Variable>{


    let (input,_) = multispace1(input)?;
    let (input,_) = tag("var")(input)?;
    let (input,_) = multispace1(input)?;
    let (input,x) = alphanumeric1(input)?;
    let (input,_) = multispace1(input)?;
    let (input,_) = tag(":")(input)?;
    let (input,_) = multispace1(input)?;



    let (input,value) = alt((float_par,digit_par))(input)?;


    let (input,_) =  take_until("type")(input)?;
    
    let (input,_) = tag("type")(input)?;

    let (input,_) = multispace1(input)?;
    let (input,_) = tag(":")(input)?;
    let (input,_) = multispace1(input)?;
    println!("{:?}",input);
    let (input,type_string) = alpha1(input)?;

    let (input,_) = multispace1(input)?;
   
    let dummy = Variable { name: x.to_string(), value: value, type_name: type_string.to_string() };
    println!("{:?}",dummy);
    Ok((input,dummy))
}


pub fn record_parser<T: AsRef<str>>(input:&str) -> IResult<&str,Record<&str>> {

 let  mut parser = delimited(tag("--"), recparser, tag("--"));

    parser(input)
}

pub fn recparser(input:&str) -> IResult<&str,Record<&str>>{

    
    let (input,_) = multispace1(input)?;
    let (input,_) = tag("record")(input)?;
    let (input,_) = multispace1(input)?;
    let (input,recordname) = alphanumeric1(input)?;

    let (input,_) =  take_until(":")(input)?;

    let (input,_) = tag(":")(input)?;
  
    let (input,_) = multispace1(input)?;

    
    let (input,fields) = fieldsparser(input)?;
    println!("*******{:?}",fields);

    let record = Record { recordname : recordname.to_string(),
        fields,

    };

    let (input,_) = multispace0(input)?;

    println!("{}",input);

    Ok((input,record))

}


fn fieldsparser(input:&str) -> IResult<&str,Vec<(&str,&str)>>{

    let mut v = Vec::new();

    let mut lines = input.lines();

    let mut last_remain = "";

    for line in lines{
        let (line,_) = multispace0(line)?;
    
        let (line,key) = alphanumeric1(line)?;
        let (line,_) = multispace0(line)?;
        let (line,_) = tag(":")(line)?;
        let (line,_) = multispace0(line)?;
        let (line,value) = alphanumeric1(line)?;
        v.push((key,value));
        last_remain = line;
        

    }

    
    println!("{:?}",v) ;

     Ok((last_remain,v))


}
