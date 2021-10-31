

use std::collections::HashMap;


mod parser;

use parser:: { varparser, uint,ftd_parser,record_parser} ;

/// Build a nom parser for ftd - https://www.fifthtry.com/ftd/

///Variables - Examples below
///
/// -- var x : 20
/// type : Integer
///
/// -- var is-monday : true
/// type : Boolean

#[derive(Debug,PartialEq,Clone)]
pub enum Ftdtype {
    Integer(i32),
    Boolean(bool),
    Decimal(f32),
    String,
}

#[derive(Debug,PartialEq,Clone)]
pub struct Variable {
    name: String,
    value: Ftdtype,
    type_name: String,
}

// #[derive(Debug,PartialEq,Clone,Hash)]
// pub struct Field{
//     key:String,
//     value:String,
// }

// impl Field{
//     fn new(key:&str,value:&str) -> Self{
//         Field{
//             key:key.to_string(),
//             value: value.to_string(),
//         }
//     }
// }

#[derive(Debug,Clone,PartialEq)]
pub struct Record<T>{
    recordname:String,
    fields:Vec<(T,T)>,
}




#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_tagvar() {
        assert_eq!(
            ftd_parser("-- var x : 10.2 
             type : Decimal --"),
            Ok((
                "",
                Variable {
                    name: 'x'.to_string(),
                    value: Ftdtype::Decimal(10.2),
                    type_name: "Decimal".to_string(),
                }
            ))
        );
    }


    #[test]
    fn test_record() {

        let mut fields = Vec::new() ;
        fields.push(("name", "caption"));
        fields.push(("age", "Integer"));
        fields.push(("bio","body"));


        assert_eq!(
             record_parser::<&str> ("-- record person:
             name : caption
             age  : Integer
             bio :  body --"),
            Ok((
                "",
                Record {
                    recordname: "person".to_string(),
                    fields: fields ,
                }
            ))
        );
    }

}
