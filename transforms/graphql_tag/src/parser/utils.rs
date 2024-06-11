use apollo_parser::{cst::OperationType, Error, Lexer, TokenKind};

use swc_ecma_ast::*;

pub fn get_key_value_node(key: String, value: Expr) -> PropOrSpread {
    PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Str(key.into()),
        value: Box::new(value),
    })))
}

pub fn get_operation_token(operation_type: Option<OperationType>) -> String {
    if operation_type.is_none() { 
        return "query".into();
    }
    
    let opr_token = operation_type.unwrap();

    if opr_token.query_token().is_some() {
        return opr_token.query_token().unwrap().text().into();
    }

    if opr_token.mutation_token().is_some() {
        return opr_token.mutation_token().unwrap().text().into();
    }

    if opr_token.subscription_token().is_some() {
        return opr_token.subscription_token().unwrap().text().into();
    }

    "query".into()
}

fn is_punctuator_token_kind(kind: TokenKind) -> bool {
    match kind {
        TokenKind::Bang
        | TokenKind::Dollar
        | TokenKind::Amp
        | TokenKind::Spread
        | TokenKind::Comma
        | TokenKind::Colon
        | TokenKind::Eq
        | TokenKind::At
        | TokenKind::LParen
        | TokenKind::RParen
        | TokenKind::LBracket
        | TokenKind::RBracket
        | TokenKind::LCurly
        | TokenKind::RCurly
        | TokenKind::Pipe
        | TokenKind::Eof => true,
        _ => false,
    }
}

pub fn strip_ignored_characters(source: String) -> Result<String, Vec<Error>> {
    let lexer = Lexer::new(source.as_str());

    let mut stripped_body = String::new();
    let mut was_last_added_token_non_punctuator = false;
    let (tokens, errors) = lexer.lex();

    if errors.len() != 0 {
        return Err(errors);
    }

    for token in tokens {
        let kind = token.kind();
        match kind {
            TokenKind::Whitespace | TokenKind::Comment | TokenKind::Eof => continue,
            _ if !is_punctuator_token_kind(kind) => {
                if was_last_added_token_non_punctuator && kind != TokenKind::Spread {
                    stripped_body += " ";
                }
                stripped_body += token.data();
                was_last_added_token_non_punctuator = true;
            }
            _ => {
                stripped_body += token.data();
                was_last_added_token_non_punctuator = false;
            }
        }
    }

    Ok(stripped_body)
}
