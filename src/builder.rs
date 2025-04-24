use raug::prelude::*;

use crate::widgets::Widget;

pub trait IcedGraph {
    fn add_widget<T: Widget>(&self, widget: T) -> (T, Vec<Node>);
}

impl IcedGraph for Graph {
    fn add_widget<T: Widget>(&self, widget: T) -> (T, Vec<Node>) {
        let nodes = widget.add_params(self);
        (widget, nodes)
    }
}
