
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









// fn ftdtype_int (input: &str) -> IResult<&str,&str>
// {

//     // let (res,kind) = alt((tag("type"),tag("type")))(input)?;
//     // Ok((res,kind))

//     // peek(digit1)
//     recognize(
//         alt((
//             double,uint
//         ))
//     )(input)
   
// }
fn inner_parser(input: &str) -> IResult<&str, bool> {
    value(false, tag("type"))(input)
}

pub fn ftd_parser(input:&'static str) -> IResult<&str,Variable>{
    let  mut parser = delimited(tag("--"), varparser, tag("--"));

    parser(input)
}

pub fn varparser(input:&'static str) -> IResult<&'static str,Variable>{
    // let mut val = Ftdtype::Integer;

    // let (input,_) = tag("--")(input)?;

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

    //  let mut p = recognize(inner_parser);

    //  let (type_found,output) = p(input)?;

    //  println!("input = {},x = {:?}",type_found,output);

    // let(input,value) = ftdtype_int(input)?;

    // let  = take_while(tag);

    //  let b  =   peek(tag("type"))(input)?;

    // let b = alt((float,digit1))(input)?;
//     println!("{:?}",input);
//     //let mut parse1 : impl FnMut(&str) -> Result<(&str,i32),nom::Err<_>> = map_opt(digit1, |s: &str| s.parse::<i32>().ok());
//     let mut parse1  = map(digit1, |s: &str| s.parse::<i32>());

//   //  let (input,(s1,s2)) = consumed(parse_ftdtype(input))(input)?;

    //  println!("{:?}, remain = {}",input,value);
//     let (input1,num) = parse1(input)? ;
//         val = Integer(num.unwrap());
    // let (input,remain) = value(,digit1(input)) ?;

    // println!("{}",input);
    let (input,_) = multispace1(input)?;
    //  let (input,remain) = newline(input)?;


    
    // let (input,_) = tag("type")(input)?;


    // let (input,_) = multispace1(input)?;
    // if let Ok(value) =  input.parse::<i32>() {
    //     val = Integer(value) ;
    // }else if let Ok(value) = input.parse::<f32>(){
    //     val = Decimal(value);
    // }

    // let ((input,value)) = alt((digit1(input)?,float(input)?))  ;
    
    // println!("{:?}, input1 = {}",input,input1);
    
    let dummy = Variable { name: x.to_string(), value: value, type_name: "Integer".to_string() };

    Ok(("",dummy))
}

