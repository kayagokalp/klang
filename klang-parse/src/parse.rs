use crate::token::Token;
use klang_ast::node::ASTNode;

pub type ParsingResult = Result<(Vec<ASTNode>, Vec<Token>), String>;

pub(crate) enum PartParsingResult<T> {
    Good(T, Vec<Token>),
    NotComplete,
    Bad(String),
}

/// TODO REMOVE THIS
pub(crate) fn error<T>(message: &str) -> PartParsingResult<T> {
    PartParsingResult::Bad(message.to_string())
}

#[macro_export]
macro_rules! expect_token (
    ([ $($token:pat, $value:expr, $result:stmt);+ ] <= $tokens:ident, $parsed_tokens:ident, $error:expr) => (
        match $tokens.pop() {
            $(
                Some($token) => {
                    $parsed_tokens.push($value);
                    $result
                },
             )+
             None => {
                 $parsed_tokens.reverse();
                 $tokens.extend($parsed_tokens.into_iter());
                 return PartParsingResult::NotComplete;
             },
            _ => return error($error)
        }
    );

    ([ $($token:pat, $value:expr, $result:stmt);+ ] else $not_matched:block <= $tokens:ident, $parsed_tokens:ident) => (
        match $tokens.last().map(|i| {i.clone()}) {
            $(
                Some($token) => {
                    $tokens.pop();
                    $parsed_tokens.push($value);
                    $result
                },
             )+
            _ => {$not_matched}
        }
    )
);

#[macro_export]
macro_rules! parse_try(
    ($function:ident, $tokens:ident, $parsed_tokens:ident) => (
        parse_try!($function, $tokens, $parsed_tokens,)
    );

    ($function:ident, $tokens:ident, $parsed_tokens:ident, $($arg:expr),*) => (
        match $function {
            PartParsingResult::Good(ast, toks) => {
                $parsed_tokens.extend(toks.into_iter());
                ast
            },
            PartParsingResult::NotComplete => {
                $parsed_tokens.reverse();
                $tokens.extend($parsed_tokens.into_iter());
                return PartParsingResult::NotComplete;
            },
            PartParsingResult::Bad(message) => return PartParsingResult::Bad(message)
        }
    )
);

pub(crate) trait Parse<T> {
    fn parse(tokens: &mut Vec<Token>) -> PartParsingResult<T>;
}
