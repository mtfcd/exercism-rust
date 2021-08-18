pub mod graph {
    use std::collections::HashMap;

    use self::graph_items::{edge::Edge, node::Node};

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes = nodes.into();
            self
        }
        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges = edges.into();
            self
        }
        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    attrs.iter().for_each(|attr| {
                        self.attrs.insert(attr.0.into(), attr.1.into());
                    });
                    self
        }
        pub fn get_node(&self, name: &str) -> Option<&Node> {
            self.nodes.iter().find(|node|node.name == name) 
        }
    }

    pub mod graph_items {
        pub mod node {
            use std::collections::HashMap;

            #[derive(Clone, PartialEq, Eq, Debug)]
            pub struct Node {
                pub name: String,
                attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Self {
                        name: name.into(),
                        attrs: HashMap::new()
                    }
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    attrs.iter().for_each(|attr| {
                        self.attrs.insert(attr.0.into(), attr.1.into());
                    });
                    self
                }
                pub fn get_attr(&self, name: &str) -> Option<&str> {
                    self.attrs.get(name).map(|value| value.as_str())
                }
            }
        }
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Clone, PartialEq, Eq, Debug)]
            pub struct Edge {
                a: String,
                b: String,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(a: &str, b: &str) -> Self {
                    Self {
                        a: a.into(),
                        b: b.into(),
                        attrs: HashMap::new()
                    }
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    attrs.iter().for_each(|attr| {
                        self.attrs.insert(attr.0.into(), attr.1.into());
                    });
                    self
                }
            }
        }
    }
}
