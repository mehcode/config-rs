use nom::*;
use std::str::{FromStr, from_utf8};

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Expression {
    Identifier(String),
    Child(Box<Expression>, String),
    Subscript(Box<Expression>, i32),
}

named!(ident_<String>,
    map!(
        map_res!(is_a!(
            "abcdefghijklmnopqrstuvwxyz \
             ABCDEFGHIJKLMNOPQRSTUVWXYZ \
             0123456789 \
             _-"
        ), from_utf8),
        |s: &str| {
            s.to_string()
        }
    )
);

named!(integer <i32>,
    map_res!(
        map_res!(
            ws!(digit),
            from_utf8
        ),
        FromStr::from_str
    )
);

named!(ident<Expression>, map!(ident_, Expression::Identifier));

fn postfix(expr: Expression) -> Box<Fn(&[u8]) -> IResult<&[u8], Expression>> {
    return Box::new(move |i: &[u8]| {
        alt!(i,
            do_parse!(
                tag!(".") >>
                id: ident_ >>
                (Expression::Child(Box::new(expr.clone()), id))
            ) |
            delimited!(
                char!('['),
                do_parse!(
                    negative: opt!(tag!("-")) >>
                    num: integer >>
                    (Expression::Subscript(
                        Box::new(expr.clone()),
                        num * (if negative.is_none() { 1 } else { -1 })
                    ))
                ),
                char!(']')
            )
        )
    });
}

fn expr(input: &[u8]) -> IResult<&[u8], Expression> {
    match ident(input) {
        IResult::Done(mut rem, mut expr) => {
            while rem.len() > 0 {
                match postfix(expr)(rem) {
                    IResult::Done(rem_, expr_) => {
                        rem = rem_;
                        expr = expr_;
                    }

                    // Forward Incomplete and Error
                    result @ _ => {
                        return result;
                    }
                }
            }

            IResult::Done(&[], expr)
        }

        // Forward Incomplete and Error
        result @ _ => result,
    }
}

impl FromStr for Expression {
    type Err = ErrorKind;

    fn from_str(s: &str) -> Result<Expression, ErrorKind> {
        expr(s.as_bytes()).to_result()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::Expression::*;

    #[test]
    fn test_id() {
        let parsed: Expression = "abcd".parse().unwrap();
        assert_eq!(parsed, Identifier("abcd".into()));
    }

    #[test]
    fn test_id_dash() {
        let parsed: Expression = "abcd-efgh".parse().unwrap();
        assert_eq!(parsed, Identifier("abcd-efgh".into()));
    }

    #[test]
    fn test_child() {
        let parsed: Expression = "abcd.efgh".parse().unwrap();
        let expected = Child(Box::new(Identifier("abcd".into())), "efgh".into());

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_subscript() {
        let parsed: Expression = "abcd[12]".parse().unwrap();
        let expected = Subscript(Box::new(Identifier("abcd".into())), 12);

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_subscript_neg() {
        let parsed: Expression = "abcd[-1]".parse().unwrap();
        let expected = Subscript(Box::new(Identifier("abcd".into())), -1);

        assert_eq!(parsed, expected);
    }
}
