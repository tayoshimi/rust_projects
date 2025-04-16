// Fruchterman-Reingold アルゴリズムによるノードレイアウト
// https://mfumi.hatenadiary.org/entry/20140213/1392287682

use crate::node_manager::{Node, NodeManager};

use pyxel::Pyxel;

use petgraph;
use petgraph::Direction;
use petgraph::graph::NodeIndex;
use petgraph::prelude::Dfs;
use petgraph::visit::EdgeRef;

use std::collections::HashMap;


// ノードの位置を計算する関数
fn calc_pos(node_manager: &NodeManager, depth: usize, idx: usize, total_nodes_at_depth: usize) -> (f64, f64) {
    let horizontal_space = node_manager.world_w / (total_nodes_at_depth + 1) as f64;
    let x = horizontal_space * (idx + 1) as f64;
    let y = 10.0 + (Node::NORMAL_H + NodeManager::NODE_SPACE) * depth as f64;
    (x, y)
}

pub fn setup_tree_layout(node_manager: &mut NodeManager) {
    let root_nx = NodeIndex::new(0);
    node_manager.graph[root_nx].depth = 0;

    // 深さを設定
    let mut max_depth: usize = 0;
    let mut dfs = Dfs::new(&node_manager.graph, root_nx);
    while let Some(nx) = dfs.next(&node_manager.graph) {
        if let Some(parent_index) = node_manager.graph.neighbors_directed(nx, Direction::Incoming).next() {
            let depth = node_manager.graph[parent_index].depth + 1;
            node_manager.graph[nx].depth = depth;
            if max_depth < depth {
                max_depth = depth;
            }
        }
    }

    // 各深さのノード数を計算
    let mut nodes_per_depth: HashMap<usize, usize> = HashMap::new();
    for node in node_manager.graph.node_indices() {
        let depth = node_manager.graph[node].depth;
        *nodes_per_depth.entry(depth).or_insert(0) += 1;
    }

    // 各深さのノードを配置
    for depth in 0..=max_depth {
        let nxs: Vec<_> = node_manager.graph.node_indices()
            .filter(|nx| node_manager.graph[*nx].depth == depth)
            .collect();
        let total_nodes_at_depth = nodes_per_depth[&depth];

        for (j, j_nx) in nxs.into_iter().enumerate() {
            let (x, y) = calc_pos(node_manager, depth, j, total_nodes_at_depth);
            let node = &mut node_manager.graph[j_nx];
            node.set_pos(x, y);
        }
    }
}

// 2点間の距離を計算する関数
fn calc_distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

// スプリングレイアウトを設定する関数
pub fn setup_spring_layout(node_manager: &mut NodeManager) {
    let width = node_manager.world_w;  // ワールドの幅を使用
    let height = node_manager.world_h; // ワールドの高さを使用
    
    // 理想的なノード間距離 k を計算
    let k = 0.2 * (width * height / node_manager.graph.node_count() as f64).sqrt();
    let mut temperature = width / 10.0; // 初期温度
    let mut positions: HashMap<NodeIndex, (f64, f64)> = HashMap::new();

    // 1. ノードの初期位置をランダムに設定
    for node in node_manager.graph.node_indices() {
        let x = Pyxel::rndf(0.0, width as f64);
        let y = Pyxel::rndf(0.0, height as f64);
        positions.insert(node, (x, y));
    }

    // 2. イテレーションを繰り返してレイアウトを調整
    for _ in 0..200 {
        // 力を保持するマップを初期化
        let mut disp: HashMap<NodeIndex, (f64, f64)> = HashMap::new();
        for node in node_manager.graph.node_indices() {
            disp.insert(node, (0.0, 0.0));
        }

        // 3. ノード間の反発力を計算
        for u in node_manager.graph.node_indices() {
            for v in node_manager.graph.node_indices() {
                if u != v {
                    let delta_x = positions[&u].0 - positions[&v].0;
                    let delta_y = positions[&u].1 - positions[&v].1;
                    let distance = calc_distance(positions[&u].0, positions[&u].1, positions[&v].0, positions[&v].1);
                    if distance > 0.0 {
                        let repulsive_force = k.powi(2) / distance; // 反発力: k^2 / distance
                        disp.get_mut(&u).unwrap().0 += delta_x / distance * repulsive_force;
                        disp.get_mut(&u).unwrap().1 += delta_y / distance * repulsive_force;
                    }
                }
            }
        }

        // 4. エッジによる引力を計算
        for edge in node_manager.graph.edge_references() {
            let u = edge.source();
            let v = edge.target();
            let delta_x = positions[&u].0 - positions[&v].0;
            let delta_y = positions[&u].1 - positions[&v].1;
            let distance = calc_distance(positions[&u].0, positions[&u].1, positions[&v].0, positions[&v].1);
            if distance > 0.0 {
                let attractive_force = distance.powi(2) / k; // 引力: distance^2 / k
                disp.get_mut(&u).unwrap().0 -= delta_x / distance * attractive_force;
                disp.get_mut(&u).unwrap().1 -= delta_y / distance * attractive_force;
                disp.get_mut(&v).unwrap().0 += delta_x / distance * attractive_force;
                disp.get_mut(&v).unwrap().1 += delta_y / distance * attractive_force;
            }
        }

        // 5. ノードの位置を更新
        for node in node_manager.graph.node_indices() {
            let dx = disp[&node].0;
            let dy = disp[&node].1;
            let disp_length = (dx.powi(2) + dy.powi(2)).sqrt();
            if disp_length > 0.0 {
                let new_x = positions[&node].0 + dx / disp_length * temperature.min(disp_length);
                let new_y = positions[&node].1 + dy / disp_length * temperature.min(disp_length);
                // 位置を境界内に制限
                positions.get_mut(&node).unwrap().0 = new_x.max(0.0).min(width);
                positions.get_mut(&node).unwrap().1 = new_y.max(0.0).min(height);
            }
        }

        // 6. 温度を下げる
        temperature *= 0.9;
    }

    // 7. 最終的な位置をノードに設定
    for node in node_manager.graph.node_indices() {
        let pos = positions[&node];
        node_manager.graph[node].set_pos(pos.0, pos.1);
    }
}
