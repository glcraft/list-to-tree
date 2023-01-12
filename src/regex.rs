use crate::node::Node;

#[derive(Debug)]
pub struct Regex(pub String);

impl Regex {
    fn push_from_node(str: &mut String, node: &Node) {
        match node {
            Node::Leaf(word) => {
                str.push_str(word);
            }
            Node::Branch(word, nodes) => {
                str.push_str(word);
                str.push_str("(?:");
                let mut has_empty = false;
                for node in nodes {
                    let str_length = str.len();
                    Self::push_from_node(str, node);
                    if str_length == str.len() {
                        has_empty = true;
                        continue;
                    }
                    str.push('|');
                }
                if !has_empty {
                    str.pop();
                }
                str.push(')');
            }
        }
    }
    fn compute_length(node: &Node) -> usize {
        match node {
            Node::Leaf(word) => word.len(),
            Node::Branch(word, nodes) => {
                let mut length = word.len() + 3;
                for node in nodes {
                    length += Self::compute_length(node)+1;
                }
                length
            }
        }
    }
}

impl From<Node> for Regex {
    fn from(node: Node) -> Self {
        let mut regex = String::with_capacity(Self::compute_length(&node));
        Self::push_from_node(&mut regex, &node);
        Self(regex)
    }
}