use std::collections::HashMap;

static mut ST: SymbolTable = HashMap::new();

pub type SymbolTable = HashMap<String, Node>;

pub enum Stmt {
    Assign(Node),
    Match(Node)
}

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
            op: NodeType::Id,
            ..self.clone()
        }
    }

    pub fn create_variable(&self, value: &str) -> Node {
        Node {
            value: Some(value.to_owned()),

            ..self.clone()
        }
    }

    // creates a new
    fn update_accepts(&self, accepts: Vec<String>) -> Node {
        Node {
            accepts,
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
    fn get_states(&self) -> Node {
        match self.op {
            NodeType::Star => {
                let new = self.edges[0].get_states();
                Node { edges: vec![new], ..self.clone() }
            },
            NodeType::Concat => {
                let r_new = self.edges[1].get_states();
                Node { edges: vec![self.edges[0].clone(), r_new], ..self.clone() }
            },
            NodeType::Union => {
                let l_new = self.edges[0].get_states();
                let r_new = self.edges[1].get_states();
                Node { edges: vec![l_new, r_new], ..self.clone() }
            },
            NodeType::Str => self.set_accept(),
            NodeType::Id => self.to_owned(),
        }
    }

    fn get_accepts(&self) -> Node {
        match self.op {
            NodeType::Star => {
                let edge = self.edges[0].get_accepts();
                let accepts = edge.accepts.clone();
                Node { edges: vec![edge], accepts, ..self.clone() }
            },
            NodeType::Concat => {
                let left = self.edges[0].get_accepts();
                let right = self.edges[1].get_accepts();
                let accepts = left.accepts.clone();
                Node { edges: vec![left, right], accepts, ..self.clone() }
            },
            NodeType::Union => {
                let left = self.edges[0].get_accepts();
                let right = self.edges[1].get_accepts();
                let accepts = [left.accepts.clone(), right.accepts.clone()].concat();
                Node { edges: vec![left, right], accepts, ..self.clone() }
            },
            NodeType::Str => self.clone(),
            NodeType::Id => {
                // ST.insert(self.value.clone().unwrap(), self.edges[0].clone());
                self.update_accepts(vec![self.value.clone().unwrap()]) // TODO: implement a symbol table
                // let lookup = ST.get(&self.value.unwrap());
                // let get = ST.get(&self.value.unwrap()).expect("Undefined Variable Error");
                // return self.update_accepts(get);
            },
        }
    }

    pub fn build_state_machine(&self) -> Node {
        let accepts = self.get_accepts();
        accepts.get_states()
    }

    #[allow(unused)]
    pub fn rex_match(&self, s: String) -> bool {
        let pos = if string_compare(&self.accepts, &s) >= 0 {
            string_compare(&self.accepts, &s) as usize
        } else {
            return false;
        };
        let n = self.accepts[pos].len();
        match self.op {
            NodeType::Star => {
                if s[n..].is_empty() {
                    return true;
                }
                self.rex_match(s[n..].to_string())
            },
            NodeType::Concat => {
                if pos != 0 {
                    return false;
                } else {
                    self.edges[1].rex_match(s[n..].to_string())
                }
            },
            NodeType::Union => {
                self.edges[pos].rex_match(s)
            },
            NodeType::Str => true,
            NodeType::Id => {
                true
            }
        }
    }

    pub fn add_stmt_list(self, sl: Node) -> Node {
        Node {
            edges: vec![sl],
            ..self
        }
    }
}

pub enum Error {
    UndefinedVariable(String),
}

fn add_var(name: &str, matches: Node) -> Node {
    let node = Node {
        accepts: Vec::new(),
        edges: vec![matches.clone()],
        value: Some(name.to_owned()),
        state: None,
        op: NodeType::Id,
    }.get_accepts();
    unsafe {
        ST.insert(name.to_owned(), node.clone());
    }
    node
}

fn string_compare(accepts: &Vec<String>, s: &String) -> isize {
    // returns index of matching string, -1 otherwise
    // improve this is Option or proper type
    for (i, accept) in accepts.iter().enumerate() {
        if s.starts_with(accept) {
            return i as isize;
        }
    }
    -1
}
