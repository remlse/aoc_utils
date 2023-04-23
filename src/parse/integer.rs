//! Nom-style parsers for integer literals.

use nom::{
    character::complete::{char, digit1},
    combinator::{map_res, opt, recognize},
    sequence::preceded,
    IResult,
};

macro_rules! unsigned_int {
    ($($int: ident)+) => {
        $(
            /// nom-style parser for an integer literal
            pub fn $int(input: &str) -> IResult<&str, $int> {
                map_res(digit1, str::parse)(input)
            }
        )+
    };
}

unsigned_int!(u8 u16 u32 u64 u128);

fn signed_int(input: &str) -> IResult<&str, &str> {
    preceded(opt(char('-')), digit1)(input)
}

macro_rules! signed_int {
    ($($int: ident)+) => {
        $(
            /// nom-style parser for an integer literal
            pub fn $int(input: &str) -> IResult<&str, $int> {
                map_res(recognize(signed_int), str::parse)(input)
            }
        )+
    };
}

signed_int!(i8 i16 i32 i64 i128);

#[test]
fn test_i32() {
    assert_eq!(i32("1234"), Ok(("", 1234)));
    assert_eq!(i32("-1234"), Ok(("", -1234)));
}
