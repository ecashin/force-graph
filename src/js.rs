use anyhow::{Context, Result};
use js_sys::Array;
use ndarray::{Array2, Axis};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct ColorSpec {
    color: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct NodeProperties {
    id: u32,
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
    pub fn new_node(pos: Array, node_properties: JsValue) -> Node;
    #[wasm_bindgen(method, js_name="addTo", js_class=Node)]
    pub fn add_node_to_graph(this: &Node, graph: &Graph);
    #[wasm_bindgen(method, js_name = "setPos")]
    pub fn set_pos(this: &Node, pos: Array);

    #[wasm_bindgen(method, js_name="addTo", js_class=Edge)]
    pub fn add_edge_to_graph(this: &Edge, graph: &Graph);

    #[wasm_bindgen(method, js_name="renderIn", js_class=Graph)]
    pub fn render_in_element(this: &Graph, element: &str);
    #[wasm_bindgen(method, structural, indexing_getter)]
    fn get(this: &Graph, prop: &str) -> Array;
    #[wasm_bindgen(method)]
    pub fn nodes(this: &Graph) -> Array;
    #[wasm_bindgen(method)]
    pub fn node(this: &Graph, id: u32) -> Node;
    #[wasm_bindgen(method, js_name=purgeNodes)]
    pub fn purge_nodes(this: &Graph);
    #[wasm_bindgen(method, js_name=addNodes)]
    pub fn add_nodes(this: &Graph, nodes: Array);
    #[wasm_bindgen(method, js_name=syncDataToFrames)]
    pub fn sync_data(this: &Graph);
}

/*
fn add_remove_nodes(graph: &Graph) {
    let nodes = graph.nodes();
    // let edges = graph.edges();
    graph.purge_nodes();
    graph.add_nodes(nodes);
}
*/

pub fn graph_update_positions(graph: &Graph, pos: &Array2<f32>) {
    for i in 0..pos.len_of(Axis(0)) {
        let js_row = Array::new();
        for j in 0..pos.len_of(Axis(1)) {
            js_row.push(&JsValue::from(pos[[i, j]] as f64));
        }
        let node = graph.node(i as u32);
        console_log(JsValue::from(&node));
        node.set_pos(js_row);
    }
    // adds new canvas: graph.render_in_element("graph");
    // add_remove_nodes(graph);
    graph.sync_data();
}

pub fn make_graph(pos: &Array2<f32>, edges: &Vec<crate::glayout::Edge>) -> Result<Graph> {
    let graph = new_graph();
    let mut i = 0;
    for row in pos.rows() {
        let point: Array = row.to_vec().iter().copied().map(JsValue::from).collect();
        let node_properties = JsValue::from_serde(&NodeProperties {
            id: i,
            color: "blue".to_owned(),
        })
        .with_context(|| "creating node properties")?;
        let node = new_node(point, node_properties);
        node.add_node_to_graph(&graph);
        i += 1;
    }
    {
        let point: Array = vec![0.0, 0.0, 0.0].into_iter().map(JsValue::from).collect();
        let node_properties = JsValue::from_serde(&NodeProperties {
            id: i,
            color: "red".to_owned(),
        })
        .with_context(|| "creating node properties for origin")?;
        new_node(point, node_properties).add_node_to_graph(&graph);
    }
    console_log(JsValue::from(&graph));
    for edge in edges {
        let nodes = Array::new();
        nodes.push(&JsValue::from(edge.src));
        nodes.push(&JsValue::from(edge.dst));
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
    let elc_debug = js_sys::Reflect::get(
        &web_sys::window().expect("getting window"),
        &JsValue::from("elcDebug"),
    )
    .expect("getting elcDebug global");
    js_sys::Reflect::set(&elc_debug, &JsValue::from("graph"), &JsValue::from(&graph))
        .expect("saving graph in JS global");
    Ok(graph)
}
