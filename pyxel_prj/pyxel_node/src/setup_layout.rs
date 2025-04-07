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

use std::collections::HashMap;

fn calcDistance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    (x2*x2+y2*y2) - (x1*x1+y1*y1)
}

pub fn SetupTreeLayout(nodeManager: &mut NodeManager) {
    let root_nx = NodeIndex::new(0);
    nodeManager.graph[root_nx].depth = 0;

    // 深さを設定
    let mut max_depth: usize = 0;
    let mut dfs = Dfs::new(&nodeManager.graph, root_nx);
    while let Some(nx) = dfs.next(&nodeManager.graph) {
        if let Some(parent_index) = nodeManager.graph.neighbors_directed(nx, Direction::Incoming).next() {
            let depth = nodeManager.graph[parent_index].depth + 1;
            nodeManager.graph[nx].depth = depth;
            if max_depth < depth {
                max_depth = depth;
            }
            println!("{}, {:?}, {:?}", depth, nx, parent_index);
        }
        dfs.stack.push(nx);
    }

    for i in 0..=max_depth {
        let nxs: Vec<_> = nodeManager.graph.node_indices()
        .filter(|nx| nodeManager.graph[*nx].depth == i).collect();

        for (j, j_nx) in nxs.into_iter().enumerate() {
            println!("{}, {}, {:?}", i, j, j_nx);
            let (x, y) = calc_pos(nodeManager, i, j);

            let node = &mut nodeManager.graph[j_nx];
            node.set_pos(x, y);
        }
    }


}

fn calc_pos(nodeManager: &NodeManager, depth: usize, idx: usize) -> (f64, f64) {
    let x = nodeManager.world_w / 2.0 - (Node::NORMAL_W + NodeManager::NODE_SPACE) * (depth) as f64 + (Node::NORMAL_W + NodeManager::NODE_SPACE) * idx as f64;
    let y = 10.0 + (Node::NORMAL_H + NodeManager::NODE_SPACE) * (depth as f64);
    (x, y)
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
