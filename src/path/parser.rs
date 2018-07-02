use super::Expression;
use nom::types::CompleteStr;
use nom::{digit, ErrorKind, IResult};
use std::str::{from_utf8, FromStr};

named!(raw_ident<CompleteStr, String>,
    map!(is_a!(
        "abcdefghijklmnopqrstuvwxyz \
         ABCDEFGHIJKLMNOPQRSTUVWXYZ \
         0123456789 \
         _-"
    ), |s: CompleteStr| {
        s.to_string()
    })
);

named!(integer<CompleteStr, isize>,
    map_res!(
        ws!(digit),
        |s: CompleteStr| {
            s.parse()
        }
    )
);

named!(ident<CompleteStr, Expression>, map!(raw_ident, Expression::Identifier));

#[allow(cyclomatic_complexity)]
fn postfix(expr: Expression) -> Box<Fn(CompleteStr) -> IResult<CompleteStr, Expression>> {
    Box::new(move |i: CompleteStr| {
        alt!(
            i,
            do_parse!(tag!(".") >> id: raw_ident >> (Expression::Child(Box::new(expr.clone()), id)))
                | delimited!(
                    char!('['),
                    do_parse!(
                        negative: opt!(tag!("-")) >> num: integer
                            >> (Expression::Subscript(
                                Box::new(expr.clone()),
                                num * (if negative.is_none() { 1 } else { -1 }),
                            ))
                    ),
                    char!(']')
                )
        )
    })
}

pub fn from_str(input: &str) -> Result<Expression, ErrorKind> {
    match ident(CompleteStr(input)) {
        Ok((mut rem, mut expr)) => {
            while !rem.is_empty() {
                match postfix(expr)(rem) {
                    Ok((rem_, expr_)) => {
                        rem = rem_;
                        expr = expr_;
                    }

                    // Forward Incomplete and Error
                    result => {
                        return result.map(|(_, o)| o).map_err(|e| e.into_error_kind());
                    }
                }
            }

            Ok(expr)
        }

        // Forward Incomplete and Error
        result => result.map(|(_, o)| o).map_err(|e| e.into_error_kind()),
    }
}

#[cfg(test)]
mod test {
    use super::Expression::*;
    use super::*;

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

        let parsed: Expression = from_str("abcd.efgh.ijkl").unwrap();
        let expected = Child(
            Box::new(Child(Box::new(Identifier("abcd".into())), "efgh".into())),
            "ijkl".into(),
        );

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
