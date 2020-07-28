//use nom::bytes::complete::take_till;
//use nom::sequence::tuple;
//use nom::*;
//use nom::{
//    branch::alt,
//    bytes::complete::{escaped, tag, take_while},
//    character::complete::{alphanumeric1 as alphanumeric, char, one_of},
//    combinator::{cut, map, opt, value},
//    error::{context, convert_error, ErrorKind, ParseError, VerboseError},
//    number::complete::double,
//    sequence::{delimited, preceded, separated_pair, terminated},
//    Err, IResult,
//};
//
//use nom::character::streaming::alpha0;
//use crate::ast::Op;
//
//fn comment<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
//    delimited(tag("#"), alpha0, tag("\n"))(i)
//}
//
//fn space<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
//    let chars = " \t\r\n";
//    // nom combinators like `take_while` return a function. That function is the
//    // parser,to which we can pass the input
//    take_while(move |c| chars.contains(c))(i)
//}
//
//
//
///// root has 0-or-more ops
//fn root<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
//    i: &'a str,
//) -> IResult<&'a str, Vec<Op>, E> {
//    preceded(
//        alt!(comment, space),
//
//    )
//    delimited(
//        sp,
//        alt((map(hash, JsonValue::Object), map(array, JsonValue::Array))),
//        opt(sp),
//    )(i)
//}
