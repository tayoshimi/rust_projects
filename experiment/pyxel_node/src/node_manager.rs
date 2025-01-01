use pyxel::{Pyxel};

use petgraph;
use petgraph::Graph;
use petgraph::Direction;
use petgraph::graph::NodeIndex;
use petgraph::prelude::Dfs;
use petgraph::visit::EdgeRef;

// fn draw_text_with_border(x: f64, y: f64, s: &str, col, bcol, font, pyxel: &mut Pyxel) {
//     for dx in range(-1, 2):
//         for dy in range(-1, 2):
//             if dx != 0 or dy != 0:
//                 pyxel.text(
//                     x + dx,
//                     y + dy,
//                     s,
//                     bcol,
//                     font,
//                 )
//     pyxel.text(x, y, s, col, font);
// }

pub struct Node {
    name: String,
    x: f64,
    y: f64,
}

impl Node {
    const NORMAL_W:f64 = 40.0;
    const NORMAL_H:f64 = 18.0;
    const FONT_W:f64 = 4.0;
    const FONT_H:f64 = 8.0;

    pub fn new(name: &str,
        x: f64,
        y: f64) -> Node {
            Node {name: name.to_string(), x: x, y: y
            }

    }

    pub fn update(&mut self, pyxel: &mut Pyxel) {
    }

    pub fn move_pos(&mut self, vx: f64, vy: f64) {
        self.x = self.x + vx;
        self.y = self.y + vy;
    }

    pub fn get_center(&self) -> (f64, f64) {
        (self.x + Node::NORMAL_W / 2.0, self.y + Node::NORMAL_H / 2.0)
    }

    fn get_text_draw_pos(&mut self) -> (f64, f64) {
        let tx: f64 = self.x + (Node::NORMAL_W - Node::FONT_W * self.name.len() as f64) / 2.0;
        let ty: f64 = self.y + (Node::NORMAL_H - Node::FONT_H) / 2.0;
        (tx, ty)
    }

    pub fn draw(&mut self, pyxel: &mut Pyxel) {
        pyxel.elli(self.x, self.y, Node::NORMAL_W, Node::NORMAL_H, pyxel::COLOR_RED);
        pyxel.ellib(self.x, self.y, Node::NORMAL_W, Node::NORMAL_H, pyxel::COLOR_WHITE);
        let (tx, ty) = self.get_text_draw_pos();
        pyxel.text(tx, ty, &self.name, 10, None);
    }

}

pub struct NodeManager {
    pub graph: Graph::<Node,(),petgraph::Directed>,
    world_w: f64,
    world_h: f64,
    count: i64,
}

impl NodeManager {
    const NODE_SPACE:f64 = 20.0;

    pub fn new(
        world_w: f64,
        world_h: f64) -> NodeManager {
            let mut graph = Graph::<Node,(),petgraph::Directed>::new();
            NodeManager {
                graph: graph,
                world_w: world_w,
                world_h: world_h,
                count: 0,
            }

    }

    pub fn add_node(&mut self, name: &str) -> NodeIndex {
        let idx = self.graph.add_node(Node::new(name, 10.0, 10.0));
        // self.graph[idx].x = 10.0 + (Node::NORMAL_W + Self::NODE_SPACE) * (idx.index() as f64 % 5.0);
        // self.graph[idx].y = 10.0 + (Node::NORMAL_H + Self::NODE_SPACE) * (idx.index() as f64 / 5.0);
        self.graph[idx].x = (self.world_w - Node::NORMAL_W) / 2.0 + (Node::NORMAL_W + Self::NODE_SPACE) * (1 - (self.count % 2) * 2) as f64;
        self.graph[idx].y = 10.0 + (Node::NORMAL_H + Self::NODE_SPACE) * (idx.index() as f64 / 5.0);
        
        self.count = self.count + 1;
        idx
    }

    pub fn add_edge(&mut self, e1: usize, e2: usize) {
        self.graph.add_edge(NodeIndex::new(e1), NodeIndex::new(e2), ());
    }

    pub fn update(&mut self, pyxel: &mut Pyxel) {
    }

    pub fn draw(&mut self, pyxel: &mut Pyxel) {
        let mut dfs = Dfs::new(&self.graph, NodeIndex::new(0));
        while let Some(nx) = dfs.next(&self.graph) {
            for edge in self.graph.edges_directed(nx, Direction::Outgoing) {
                let t_nx = edge.target();
                self.draw_edge(nx, t_nx, pyxel);
            }
            dfs.stack.push(nx);
            for edge in self.graph.edges_directed(nx, Direction::Incoming) {
                dfs.stack.push(edge.source());
            }

            self.graph[nx].draw(pyxel);
        }
    }

    fn draw_edge(&self, src_nx: NodeIndex, target_nx: NodeIndex, pyxel: &mut Pyxel) {
        let (sx, sy) = self.graph[src_nx].get_center();
        let (tx, ty) = self.graph[target_nx].get_center();

        pyxel.line(sx,sy,tx,ty,pyxel::COLOR_WHITE);
    }

}
