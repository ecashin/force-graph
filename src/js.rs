use anyhow::{Context, Result};
use js_sys::Array;
use ndarray::Array2;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct ColorSpec {
    color: String,
}

#[wasm_bindgen]
extern "C" {
    pub type Edge;
    pub type Graph;
    pub type Node;

    #[wasm_bindgen(js_namespace=console, js_name=log)]
    pub fn console_log(msg: JsValue);
    #[wasm_bindgen(js_namespace=G, js_name="edge")]
    pub fn new_edge(nodes: Array, color_spec: JsValue) -> Edge;
    #[wasm_bindgen(js_namespace=G, js_name="graph")]
    pub fn new_graph() -> Graph;
    #[wasm_bindgen(js_namespace=G, js_name="node")]
    pub fn new_node(pos: Array, color_spec: JsValue) -> Node;
    #[wasm_bindgen(method, js_name="addTo", js_class=Node)]
    pub fn add_node_to_graph(this: &Node, graph: &Graph);
    #[wasm_bindgen(method, js_name="addTo", js_class=Edge)]
    pub fn add_edge_to_graph(this: &Edge, graph: &Graph);
    #[wasm_bindgen(method, js_name="renderIn", js_class=Graph)]
    pub fn render_in_element(this: &Graph, element: &str);
    #[wasm_bindgen(method, structural, indexing_getter)]
    fn get(this: &Graph, prop: &str) -> Array;
}

fn graph_node(graph: &Graph, index: usize) -> Node {
    let nodes = graph.get("_nodes");
    let node = Node::from(nodes.get(index as u32));
    console_log(JsValue::from(format!("found Node at index {}:", index)));
    console_log(JsValue::from(&node));
    node
}

pub fn make_graph(pos: Array2<f32>, edges: Vec<crate::glayout::Edge>) -> Result<Graph> {
    let graph = new_graph();
    // let mut js_nodes: Vec<JsValue> = Vec::new();
    for row in pos.rows() {
        let point: Array = row.to_vec().iter().copied().map(JsValue::from).collect();
        let color = JsValue::from_serde(&ColorSpec {
            color: "blue".to_owned(),
        })
        .with_context(|| "creating color spec")?;
        let node = new_node(point, color);
        node.add_node_to_graph(&graph);
        // js_nodes.push(JsValue::from(node));
    }
    console_log(JsValue::from(&graph));
    for edge in edges {
        let nodes = Array::new();
        nodes.push(&graph_node(&graph, edge.src));
        nodes.push(&graph_node(&graph, edge.dst));
        let js_edge = new_edge(
            nodes,
            JsValue::from_serde(&ColorSpec {
                color: "black".to_owned(),
            })
            .expect("creating color spec"),
        );
        js_edge.add_edge_to_graph(&graph);
    }
    console_log(JsValue::from("graph after adding edges:"));
    console_log(JsValue::from(&graph));
    graph.render_in_element("graph");
    Ok(graph)
}
