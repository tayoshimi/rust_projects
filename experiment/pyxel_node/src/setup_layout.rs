// Fruchterman-Reingold アルゴリズムによるノードレイアウト
// https://mfumi.hatenadiary.org/entry/20140213/1392287682

use crate::node_manager::{Node, NodeManager};

use petgraph;
use petgraph::Graph;
use petgraph::Direction;
use petgraph::graph::NodeIndex;
use petgraph::prelude::Dfs;
use petgraph::prelude::Bfs;
use petgraph::visit::EdgeRef;

fn calcDistance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    (x2*x2+y2*y2) - (x1*x1+y1*y1)
}

pub fn SetupTreeLayout(nodeManager: &NodeManager) {
    let mut bfs = Bfs::new(&nodeManager.graph, NodeIndex::new(0));
    while let Some(nx) = bfs.next(&nodeManager.graph) {
        //println!("{:?}", nx);
        for (i, edge) in nodeManager.graph.edges_directed(nx, Direction::Outgoing).enumerate() {
            let t_nx = edge.target();
        }

    }
}

/*
pub fn GravityPower(graph: Graph::<Node,(),petgraph::Directed>, power: f64) {
    let mut dfs = Dfs::new(graph, NodeIndex::new(0));
    while let Some(nx) = dfs.next(&self.graph) {
        for edge in self.graph.edges_directed(nx, Direction::Outgoing) {
            let t_nx = edge.target();
        }
        dfs.stack.push(nx);

    }
}
    */
