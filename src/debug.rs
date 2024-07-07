use std::fmt;

use crate::{hnsw::HNSW, point::Point};

impl fmt::Debug for HNSW {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut message: String = "".to_string();
        message.push_str("==========\n");
        message.push_str(&format!(
            "ef_construction: {}, layer_count: {}, node count: {}\n",
            self.ef_construction(),
            self.layer_probas().len(),
            self.points().len()
        ));
        message.push_str("==========\n");
        let mut layer = self.layer_probas().len() - 1;
        loop {
            let mut layer_nodes: Vec<Point> = Vec::new();
            for (_, node) in self.points().into_iter() {
                if node.layer_index() == layer {
                    layer_nodes.push(node);
                }
            }

            message.push_str(&format!(
                "layer: {}, node_count: {}\n",
                layer,
                layer_nodes.len()
            ));
            for node in layer_nodes {
                let neighbors = node.neighbors();
                message.push_str("----------\n");
                message.push_str(&format!(
                    "node: {}, neighbors count : {}, neighbors: |",
                    node.id(),
                    neighbors.len()
                ));
                for neighbor in neighbors {
                    message.push_str(&format!("{:?}|", neighbor))
                }
                message.push_str("\n");
            }
            message.push_str("==========\n");
            if layer == 0 {
                break;
            } else {
                layer -= 1;
            }
        }
        write!(f, "{}", message)
    }
}
