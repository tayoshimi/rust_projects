use pyxel::{Pyxel};

use petgraph;
use petgraph::Graph;
use petgraph::Direction;
use petgraph::graph::NodeIndex;
use petgraph::prelude::Dfs;
use petgraph::visit::EdgeRef;

// fn draw_text_with_border(x: f32, y: f32, s: &str, col, bcol, font, pyxel: &mut Pyxel) {
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
    x: f32,
    y: f32,
    pub depth: usize,
}

impl Node {
    pub const NORMAL_W:f32 = 40.0;
    pub const NORMAL_H:f32 = 18.0;
    pub const FONT_W:f32 = 4.0;
    pub const FONT_H:f32 = 8.0;

    pub fn new(name: &str,
        x: f32,
        y: f32) -> Node {
            Node {name: name.to_string(), x: x, y: y, depth: 0
            }

    }

    pub fn update(&mut self, pyxel: &mut Pyxel) {
    }

    pub fn set_pos(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn move_pos(&mut self, vx: f32, vy: f32) {
        self.x = self.x + vx;
        self.y = self.y + vy;
    }

    pub fn get_center(&self) -> (f32, f32) {
        (self.x + Node::NORMAL_W / 2.0, self.y + Node::NORMAL_H / 2.0)
    }

    fn get_text_draw_pos(&mut self) -> (f32, f32) {
        let tx: f32 = self.x + (Node::NORMAL_W - Node::FONT_W * self.name.len() as f32) / 2.0;
        let ty: f32 = self.y + (Node::NORMAL_H - Node::FONT_H) / 2.0;
        (tx, ty)
    }

    pub fn draw(&mut self, pyxel: &mut Pyxel) {
        pyxel.elli(self.x, self.y, Node::NORMAL_W, Node::NORMAL_H, pyxel::COLOR_RED);
        pyxel.ellib(self.x, self.y, Node::NORMAL_W, Node::NORMAL_H, pyxel::COLOR_WHITE);
        let (tx, ty) = self.get_text_draw_pos();
        pyxel.text(tx, ty, &self.name, 10, None);
        //pyxel.text(tx, ty, &self.depth.to_string(), 10, None);

    }

}

pub struct NodeManager {
    pub graph: Graph::<Node,(),petgraph::Directed>,
    pub world_w: f32,
    pub world_h: f32,
    count: i32,
}

impl NodeManager {
    pub const NODE_SPACE:f32 = 20.0;

    pub fn new(
        world_w: f32,
        world_h: f32) -> NodeManager {
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
        // self.graph[idx].y = 10.0 + (Node::NORMAL_H + Self::NODE_SPACE) * (idx.index() as f32 / 5.0);
        self.graph[idx].x = (self.world_w - Node::NORMAL_W) / 2.0 + (Node::NORMAL_W + Self::NODE_SPACE) * (1 - (self.count % 2) * 2) as f32;
        self.graph[idx].y = 10.0 + (Node::NORMAL_H + Self::NODE_SPACE) * (idx.index() as f32 / 5.0);
        
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
