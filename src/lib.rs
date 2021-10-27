

mod parser;

use parser:: { varparser, parser_cond ,uint,ftd_parser} ;

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
                    type_name: "Integer".to_string(),
                }
            ))
        );
    }

    #[test]
    fn test2(){
        assert_eq!(parser_cond(true, "abcd;"), Ok((";", Some("abcd"))));
        assert_eq!(parser_cond(false, "abcd;"), Ok(("abcd;", None)));
        // assert_eq!(parser_cond(true, "123;"), Err(Err::Error(Error::new("123;", ErrorKind::Alpha))));
        // assert_eq!(parser_cond(false, "123;"), Ok(("123;", None)));
    }



    #[test]
    fn test3(){
        assert_eq!(uint("123"), Ok(("", "123")));
        assert_eq!(uint("0"), Ok(("", "0")));
        assert_eq!(uint("-123"), Ok(("", "-123")));
        assert_eq!(uint("123e"), Ok(("e","123")));
        assert_eq!(uint("0123"), Ok(("","0123")));
    
    }
}
