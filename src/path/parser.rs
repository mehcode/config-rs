use nom::*;
use std::str::{FromStr, from_utf8};
use super::Expression;

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

named!(integer <isize>,
    map_res!(
        map_res!(
            ws!(digit),
            from_utf8
        ),
        FromStr::from_str
    )
);

named!(ident<Expression>, map!(ident_, Expression::Identifier));

#[allow(cyclomatic_complexity)]
fn postfix(expr: Expression) -> Box<Fn(&[u8]) -> IResult<&[u8], Expression>> {
    Box::new(move |i: &[u8]| {
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
    })
}

pub fn from_str(input: &str) -> Result<Expression, ErrorKind> {
    match ident(input.as_bytes()) {
        IResult::Done(mut rem, mut expr) => {
            while !rem.is_empty() {
                match postfix(expr)(rem) {
                    IResult::Done(rem_, expr_) => {
                        rem = rem_;
                        expr = expr_;
                    }

                    // Forward Incomplete and Error
                    result => {
                        return result.to_result().map_err(|e| e.into_error_kind());
                    }
                }
            }

            Ok(expr)
        }

        // Forward Incomplete and Error
        result => result.to_result().map_err(|e| e.into_error_kind()),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::Expression::*;

    #[test]
    fn test_id() {
        let parsed: Expression = from_str("abcd").unwrap();
        assert_eq!(parsed, Identifier("abcd".into()));
    }

    #[test]
    fn test_id_dash() {
        let parsed: Expression = from_str("abcd-efgh").unwrap();
        assert_eq!(parsed, Identifier("abcd-efgh".into()));
    }

    #[test]
    fn test_child() {
        let parsed: Expression = from_str("abcd.efgh").unwrap();
        let expected = Child(Box::new(Identifier("abcd".into())), "efgh".into());

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_subscript() {
        let parsed: Expression = from_str("abcd[12]").unwrap();
        let expected = Subscript(Box::new(Identifier("abcd".into())), 12);

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_subscript_neg() {
        let parsed: Expression = from_str("abcd[-1]").unwrap();
        let expected = Subscript(Box::new(Identifier("abcd".into())), -1);

        assert_eq!(parsed, expected);
    }
}
