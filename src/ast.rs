
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

    fn update_union(self) -> Node {
        Node {
            edges: Vec::from([self.edges[0].set_accept(), self.edges[1].set_accept()]),
            ..self
        }
    }

    // update function to not use clone()
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
            _ => self.to_owned(),
        }
    }
}

// create aa repeat and b;
// create a or b;
