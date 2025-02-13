use crate::prelude::*;
use biome_formatter::token::number::format_number_token;

use crate::parentheses::NeedsParentheses;
use biome_formatter::write;
use biome_js_syntax::{JsSyntaxNode, TsNumberLiteralType, TsNumberLiteralTypeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsNumberLiteralType;

impl FormatNodeRule<TsNumberLiteralType> for FormatTsNumberLiteralType {
    fn fmt_fields(&self, node: &TsNumberLiteralType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsNumberLiteralTypeFields {
            minus_token,
            literal_token,
        } = node.as_fields();
        write![
            f,
            [minus_token.format(), format_number_token(&literal_token?)]
        ]
    }

    fn needs_parentheses(&self, item: &TsNumberLiteralType) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for TsNumberLiteralType {
    fn needs_parentheses_with_parent(&self, _parent: JsSyntaxNode) -> bool {
        false
    }
}
