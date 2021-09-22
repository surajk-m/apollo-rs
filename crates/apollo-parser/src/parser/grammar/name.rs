use crate::{Parser, SyntaxKind, TokenKind, S};

/// See: https://spec.graphql.org/June2018/#Name
///
/// ```txt
/// Name
///     [_A-Za-z][_0-9A-Za-z]*/
/// ```
pub(crate) fn name(p: &mut Parser) {
    let _g = p.start_node(SyntaxKind::NAME);
    match p.peek() {
        Some(TokenKind::Name) => {
            validate_name(p.peek_data().unwrap(), p);
            p.bump(SyntaxKind::IDENT);
        }
        _ => p.err("expected a name"),
    }
}

pub(crate) fn validate_name(name: String, p: &mut Parser) {
    if !name.starts_with(is_start_char) {
        p.err("expected Name to start with a letter or an _");
    }
    if name.len() >= 2 && !name[1..].chars().all(is_remainder_char) {
        p.err("Name can only be composed of letters, numbers and _");
    }
}

/// See: https://spec.graphql.org/June2018/#Alias
///
/// ```txt
/// Alias
///     Name :
/// ```
pub(crate) fn alias(p: &mut Parser) {
    let _g = p.start_node(SyntaxKind::ALIAS);
    name(p);
    p.bump(S![:]);
}

fn is_start_char(c: char) -> bool {
    matches!(c, '_' | 'A'..='Z' | 'a'..='z')
}

fn is_remainder_char(c: char) -> bool {
    matches!(c, '_' | 'A'..='Z' | 'a'..='z' | '0'..='9')
}
