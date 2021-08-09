use ndarray::{Array2, Axis};
use yew::{html, Component, ComponentLink, Html, InputData, ShouldRender};

mod js;

enum Msg {
    SetNVertices(InputData),
    SetNDimensions(InputData),
    SetMaxDegree(InputData),
    SetNIters(InputData),
    NewGraph,
    LayoutIter,
}

struct Model {
    debug: String,
    link: ComponentLink<Self>,
    n_vertices: usize,
    n_dimensions: usize,
    max_degree: usize,
    n_iters: usize,
    graph: Option<js::Graph>,
    positions: Option<Array2<f32>>,
    edges: Option<Vec<forcegraph::Edge>>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            debug: "(unset)".to_owned(),
            link,
            n_vertices: 11,
            n_dimensions: 3,
            max_degree: 3,
            n_iters: 1,
            graph: None,
            positions: None,
            edges: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetNDimensions(input) => {
                let n = input.value.parse::<usize>();
                if let Ok(n) = n {
                    self.n_dimensions = n;
                }
                true
            }
            Msg::SetNVertices(input) => {
                let n = input.value.parse::<usize>();
                if let Ok(n) = n {
                    self.n_vertices = n;
                }
                true
            }
            Msg::SetMaxDegree(input) => {
                let n = input.value.parse::<usize>();
                if let Ok(n) = n {
                    self.max_degree = n;
                }
                true
            }
            Msg::SetNIters(input) => {
                let n = input.value.parse::<usize>();
                if let Ok(n) = n {
                    self.n_iters = n;
                }
                true
            }
            Msg::NewGraph => {
                self.debug = format!(
                    "{} {} {} {}",
                    self.n_vertices, self.n_dimensions, self.max_degree, self.n_iters
                )
                .to_owned();
                let pos = forcegraph::initial_positions(self.n_vertices, self.n_dimensions);
                let edges = forcegraph::add_edges(self.n_vertices, self.max_degree);
                self.graph = Some(js::make_graph(&pos, &edges).expect("making graph"));
                self.positions = Some(pos);
                self.edges = Some(edges);
                true
            }
            Msg::LayoutIter => {
                if self.positions.is_some() && self.edges.is_some() {
                    macro_rules! row1 {
                        ($arr:expr) => {
                            $arr.as_ref().unwrap().index_axis(Axis(0), 0)
                        };
                    }
                    let mut text = format!(
                        "{:?}",
                        // self.positions.as_ref().unwrap().index_axis(Axis(0), 0)
                        row1!(self.positions)
                    );
                    forcegraph::force_graph(
                        self.positions.as_mut().unwrap(),
                        self.edges.as_ref().unwrap(),
                        self.n_iters,
                    );
                    text += format!("=>{:?}", row1!(self.positions)).as_str();
                    self.debug = text;
                    js::graph_update_positions(
                        self.graph.as_ref().unwrap(),
                        self.positions.as_ref().unwrap(),
                    )
                }
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <div class="flex two four-800">
                    <div>
                        <p>{ self.debug.clone() }</p>
                    </div>
                    <div>
                        <label for="n_vertices">{ "Number of vertices" }</label>
                        <input type="text"
                            id="n_vertices"
                            required=true
                            oninput=self.link.callback(|e: InputData| Msg::SetNVertices(e))
                        />
                    </div>
                    <div>
                        <label for="n_dimensions">{ "Number of dimensions" }</label>
                        <input type="text"
                            id="n_dimension"
                            required=true
                            oninput=self.link.callback(|e: InputData| Msg::SetNDimensions(e))
                        />
                    </div>
                    <div>
                        <label for="max_degree">{ "Maximum degree" }</label>
                        <input type="text"
                            id="max_degree"
                            required=true
                            oninput=self.link.callback(|e: InputData| Msg::SetMaxDegree(e))
                        />
                    </div>
                    <div>
                        <label for="n_iters">{ "Number of Iterations" }</label>
                        <input type="text"
                            id="n_iters"
                            required=true
                            oninput=self.link.callback(|e: InputData| Msg::SetNIters(e))
                        />
                    </div>
                </div>
                <div class="flex two">
                    <div>
                        <button id="new_graph"
                            onclick=self.link.callback(|_| Msg::NewGraph)
                            > { "New Graph" }</button>
                    </div>
                    <div>
                        <button id="layout_iter"
                            onclick=self.link.callback(|_| Msg::LayoutIter)
                            > { "Iterate Layout" }</button>
                    </div>
                </div>
                <div id="graph">
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>()
}
