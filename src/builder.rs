use std::ops::Deref;

use raug::{
    graph::Graph,
    prelude::{GraphBuilder, Node},
};

use crate::{
    runtime::IcedRuntime,
    widgets::{IntoParamVec, Widget},
};

#[derive(Default)]
pub struct IcedGraphBuilder {
    graph: GraphBuilder,
}

impl Deref for IcedGraphBuilder {
    type Target = GraphBuilder;

    fn deref(&self) -> &Self::Target {
        &self.graph
    }
}

impl IcedGraphBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_widget<T: Widget>(&self, widget: T) -> (T, Vec<Node>) {
        let param = widget.params().clone();
        let mut nodes = Vec::new();
        for param in param.into_param_vec() {
            let node = self.add_processor(param);
            nodes.push(node);
        }
        (widget, nodes)
    }

    pub fn build(self) -> Graph {
        self.graph.build()
    }

    pub fn build_runtime<T: Widget>(self, main_widget: T) -> IcedRuntime<T> {
        IcedRuntime::new(self.build(), main_widget)
    }
}
