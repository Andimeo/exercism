pub mod graph {
    pub mod graph_items {
        pub mod node {
            use std::collections::HashMap;
            #[derive(Debug, PartialEq, Clone)]
            pub struct Node {
                name: String,
                attrs: HashMap<String, String>,
            }
            impl Node {
                pub fn new(v: &str) -> Self {
                    Node {
                        name: String::from(v),
                        attrs: HashMap::new(),
                    }
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for (k, v) in attrs {
                        self.attrs.insert(String::from(*k), String::from(*v));
                    }
                    self
                }
                pub fn get_attr(&self, attr: &str) -> Option<&str> {
                    let v = self.attrs.get(attr);
                    match v {
                        None => None,
                        Some(s) => Some(&s[..]),
                    }
                }
                pub fn get_name(&self) -> &str {
                    &self.name[..]
                }
            }
        }
        pub mod edge {
            use std::collections::HashMap;
            #[derive(Clone, PartialEq, Debug)]
            pub struct Edge {
                lnode: String,
                rnode: String,
                attrs: HashMap<String, String>,
            }
            impl Edge {
                pub fn new(l: &str, r: &str) -> Self {
                    Edge {
                        lnode: String::from(l),
                        rnode: String::from(r),
                        attrs: HashMap::new(),
                    }
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Edge {
                    for (k, v) in attrs {
                        self.attrs.insert(String::from(*k), String::from(*v));
                    }
                    self
                }
                pub fn get_attr(&self, name: &str) -> Option<&str> {
                    let v = self.attrs.get(name);
                    match v {
                        None => None,
                        Some(s) => Some(&s[..]),
                    }
                }
            }
        }
    }
    use self::graph_items::{edge::Edge, node::Node};
    use std::collections::HashMap;

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: vec![],
                edges: vec![],
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes.extend_from_slice(nodes);
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges.extend_from_slice(edges);
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            for (k, v) in attrs {
                self.attrs.insert(String::from(*k), String::from(*v));
            }
            self
        }

        pub fn get_node(&self, name: &str) -> Option<&Node> {
            for node in &self.nodes {
                if node.get_name() == name {
                    return Some(node);
                }
            }
            None
        }
    }
}
