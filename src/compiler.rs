// use std::iter::Peekable;
//
// enum Token {
//     ParenOpen,
//     ParenClose,
//     BracketOpen,
//     BracketClose,
//     Colon,
//     Value(serde_json::Value),
//     Expr,
//     Return,
//     If,
//     Else,
//     True,
//     False,
//     Or,
//     And,
//     Eq,
//     Neq,
// }
//
// fn peek_match<T: Iterator<Item = char>>(iter: &mut Peekable<T>, to: &str) -> bool {
//     // re t != next expected, the iterator exhausted 2 chars
//     let (chain, matched) =
//         to.chars()
//             .zip(iter)
//             .fold((Vec::new(), true), |(give, matching), (expect, actual)| {
//                 give.push(actual);
//                 (give, matching || expect == actual)
//             });
//
//     if matched {
//         true
//     } else {
//         iter.chain(chain)
//     }
// }
//
// fn lex(ctx: &str) -> Vec<Token> {
//     let mut result = Vec::new();
//     let mut iter = ctx.chars().peekable();
//
//     while let Some(&c) = iter.peek() {
//         match c {
//             '#' => while iter.next() != Some('\n') {},
//             ' ' => {
//                 iter.next();
//             }
//             '(' => {
//                 result.push(Token::ParenOpen);
//                 iter.next();
//             }
//             ')' => {
//                 result.push(Token::ParenClose);
//                 iter.next();
//             }
//             '{' => {
//                 result.push(Token::BracketOpen);
//                 iter.next();
//             }
//             '}' => {
//                 result.push(Token::BracketClose);
//                 iter.next();
//             }
//             ';' => {
//                 result.push(Token::Colon);
//                 iter.next();
//             }
//             '"' => {
//                 let mut chars = Vec::new();
//                 while iter.peek().filter(|x| **x == '"').is_none() {
//                     chars.push(iter.next().unwrap());
//                 }
//                 let string: String = chars.into_iter().collect();
//                 result.push(Token::Value(serde_json::Value::String(string)));
//                 iter.next();
//             }
//         }
//     }
//
//     result
// }
//
//fn conditional_then(ctx: Vec<char>) -> Op {}
//
//fn conditional(ctx: Vec<char>, mut conditions: Vec<Conditional>) -> (Vec<char>, Op) {
//    // push conditions recursively until reach something that is not else if } or else }
//    match ctx.as_slice() {
//        ['i', 'f', '(', start_expr @ ..] => {
//            match start_expr {}
//
//            conditions.push(Conditional {
//                when: unimplemented!(),
//                then: unimplemented!(),
//            });
//            conditional(rest, conditions)
//        }
//        ['e', 'l', 's', 'e', 'i', 'f', '(', when @ .., ')', '{', then @ .., '}', rest @ ..] => {
//            conditions.push(Conditional {
//                when: unimplemented!(),
//                then: unimplemented!(),
//            });
//            conditional(rest, conditions)
//        }
//        ['e', 'l', 's', 'e', '{', then @ .., '}', rest @ ..] => {
//            conditions.push(Conditional {
//                when: Node::Json(serde_json::Value::Bool(true)),
//                then: unimplemented!(),
//            });
//            (rest.to_vec(), Op::Cond { cond: conditions })
//        }
//        _ => (ctx, Op::Cond { cond: conditions }),
//    }
//}
//
//fn compile_return(ctx: Vec<char>) -> Op {
//    unimplemented!()
//}
//
//fn run(ctx: Vec<char>) -> Op {
//    // first we remove all whitespace.
//    let clean = ctx.into_iter().filter(|c| *c != ' ').collect();
//    compile(clean)
//}
//
//fn compile(ctx: Vec<char>) -> Op {
//    // first we remove all comments. this will allow us to ignore whitespace for the remainder of the program
//    // comment removal is pretty simple: find a comment, remove to end of line
//    println!("Compiling: {:?}", ctx);
//
//    match ctx.as_slice() {
//        ['\n', rest @ ..] => compile(rest.to_vec()),
//        ['#', ..] => compile(ctx.into_iter().skip_while(|c| *c != '\n').skip(1).collect()),
//        ['i', 'f', ..] => conditional(ctx.into_iter().skip(2).collect(), Vec::new()),
//        ['r', 'e', 't', 'u', 'r', 'n'] => compile_return(ctx.into_iter().skip(6).collect()),
//        [variable @ .., '=', value @ ..] => unimplemented!(),
//        other => {
//            eprintln!("{:?}", other);
//            panic!("Unknown top level statemetn");
//        }
//    };
//
//    unimplemented!()
//    // declarations
//    // # comment
//    // if conditional
//    // _variable_ = assignment
//    // return bool
//}
//
//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn test_compile() {
//        let p1 = r#"
//            # top level comment
//               ratings = bernoulliTrial(p=0.8, unit=userId);
//        "#;
//
//        run(p1.chars().collect());
//        assert_eq!(true, true);
//    }
//}
//
