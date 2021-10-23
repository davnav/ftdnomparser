

mod parser;

use parser::varparser;

/// Build a nom parser for ftd - https://www.fifthtry.com/ftd/

///Variables - Examples below
///
/// -- var x : 20
/// type : Integer
///
/// -- var is-monday : true
/// type : Boolean

#[derive(Debug,PartialEq)]
pub enum Ftdtype {
    Integer(i32),
    Boolean(bool),
    Decimal(f32),
    String,
}

#[derive(Debug,PartialEq)]
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
            varparser("-- var x : 10 \n type : Integer "),
            Ok((
                "",
                Variable {
                    name: 'x'.to_string(),
                    value: Ftdtype::Integer(10),
                    type_name: "Integer".to_string()
                }
            ))
        );
    }
}
