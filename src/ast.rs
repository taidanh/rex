
#[derive(Clone,Debug)]
pub enum NodeType {
    Star,
    Concat,
    Union,
    Str,
    Id,
}

#[derive(Clone,Debug)]
pub enum NodeState {
    Start,
    Accept,
}

// v remove later
#[allow(unused)]
#[derive(Clone,Debug)]
pub struct Node {
    accepts: Vec<String>,
    edges: Vec<Node>,
    value: Option<String>,
    state: Option<NodeState>,
    op: NodeType,
}

impl Node {
    pub fn new(edges: &[Node], op: NodeType) -> Self {
        Self {
            accepts: Vec::new(),
            edges: Vec::from(edges),
            value: None,
            state: None,
            op
        }
    }

    pub fn new_str(s: &str) -> Node {
        Node {
            accepts: vec![s.to_string()],
            edges: Vec::new(),
            value: Some(s.to_string()),
            state: None,
            op: NodeType::Str
        }
    }

    fn set_accept(&self) -> Node {
        Node {
            state: Some(NodeState::Accept),
            ..self.clone()
        }
    }

    pub fn set_value(&self, value: &str) -> Node {
        Node {
            value: Some(value.to_string()),
            ..self.clone()
        }
    }

    // TODO update function to not use clone()
    pub fn set_states(&self) -> Node {
        match self.op {
            NodeType::Star => {
                let new = self.edges[0].set_states();
                new.set_accept()
            },
            NodeType::Concat => {
                let r_new = self.edges[1].set_states();
                Node { edges: vec![self.edges[0].clone(), r_new], ..self.clone() }
            },
            NodeType::Union => {
                let l_new = self.edges[0].set_states();
                let r_new = self.edges[1].set_states();
                Node { edges: vec![l_new, r_new], ..self.clone() }
            },
            NodeType::Str => self.set_accept(),
            NodeType::Id => self.to_owned(),
        }
    }
}

pub fn check_str(_s: &str) -> bool {
    todo!()
    // move CREATE LET etc out of lalrpop match and
    // into a token action?
}

// create aa repeat and b;
// create a or b;
