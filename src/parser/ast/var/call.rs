
pub struct NodeVarCall {
    name: String,
}

impl NodeVarCall {
    pub fn new(token: &Token) -> Option<NodeVarCall> {
        if let TokenKind::Identifier(val) = token.get_kind() {
            return Some( NodeVarCall { name: val.clone() });
        }
        None
    }
}

