use crate::node::Node;

#[derive(Debug)]
pub struct Regex(pub String);

impl Regex {
    fn push_from_nodes(str: &mut String, nodes: &[Node]) -> usize
    {
        let mut nword = 0;
        for node in nodes {
            let str_length = str.len();
            Self::push_from_node(str, node);
            if str_length == str.len() {
                continue;
            }
            nword += 1;
            str.push('|');
        }
        str.pop();
        nword
    }

    fn push_from_node(str: &mut String, node: &Node) {
        match node {
            Node::Leaf(word) => {
                str.push_str(word);
            }
            Node::Branch(word, nodes) => {
                str.push_str(word);
                str.push_str("(?:");
                Self::push_from_nodes(str, nodes.as_slice());
                str.push(')');
            }
            Node::BranchIncluded(word, nodes) => {
                str.push_str(word);
                str.push_str("(?:");
                Self::push_from_nodes(str, nodes.as_slice());
                str.push_str(")?");
            }
        }
    }
    fn compute_length_from_nodes(nodes: &[Node]) -> usize {
        let mut length = 0;
        for node in nodes {
            length += Self::compute_length_from_node(node)+1;
        }
        length-1
    }
    fn compute_length_from_node(node: &Node) -> usize {
        match node {
            Node::Leaf(word) => word.len(),
            Node::Branch(word, nodes) => word.len() + 4 + Self::compute_length_from_nodes(nodes.as_slice()),
            Node::BranchIncluded(word, nodes) => word.len() + 5 + Self::compute_length_from_nodes(nodes.as_slice())
        }
    }
}

impl From<Node> for Regex {
    fn from(node: Node) -> Self {
        match node {
            Node::Leaf(word) => Self(word.to_string()),
            ref node @ Node::BranchIncluded(_,_) => {
                let mut regex = String::with_capacity(Self::compute_length_from_node(&node));
                Self::push_from_node(&mut regex, &node);
                Self(regex)
            }
            ref node @ Node::Branch(ref word, ref children) => {
                if word.is_empty() {
                    let mut regex = String::with_capacity(Self::compute_length_from_nodes(children.as_slice()));
                    Self::push_from_nodes(&mut regex, children.as_slice());
                    Self(regex)
                } else {
                    let mut regex = String::with_capacity(Self::compute_length_from_node(&node));
                    Self::push_from_node(&mut regex, &node);
                    Self(regex)
                }
            }
        }
        
    }
}