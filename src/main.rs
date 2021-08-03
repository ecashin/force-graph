use yew::{html, Component, ComponentLink, Html, InputData, ShouldRender};

mod glayout;
mod js;

enum Msg {
    SetNVertices(InputData),
    SetNDimensions(InputData),
    SetMaxDegree(InputData),
    SetNIters(InputData),
    NewGraph,
    Layout,
}

struct Model {
    debug: String,
    link: ComponentLink<Self>,
    n_vertices: usize,
    n_dimensions: usize,
    max_degree: usize,
    n_iters: usize,
    graph: Option<js::Graph>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            debug: "(unset)".to_owned(),
            link,
            n_vertices: 9,
            n_dimensions: 3,
            max_degree: 3,
            n_iters: 1,
            graph: None,
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
                let pos = crate::glayout::initial_positions(self.n_vertices, self.n_dimensions);
                let edges = crate::glayout::add_edges(self.n_vertices, self.max_degree);
                self.graph = Some(js::make_graph(pos, edges).expect("making graph"));
                true
            }
            Msg::Layout => {
                // crate::glayout::force_graph(&mut pos, &edges, self.n_iters);
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
                <div class="flex">
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
                    <div>
                        <button id="new_graph"
                            onclick=self.link.callback(|_| Msg::NewGraph)
                            > { "New Graph" }</button>
                    </div>
                </div>
                <div id="graph">
                </div>
            </div>
        }
    }
}

fn main() {
    /*
    let mut pos = crate::glayout::initial_positions(n, d);
    let edges = crate::glayout::add_edges(n, m);
    crate::glayout::force_graph(&mut pos, &edges);

    eprintln!("n:{} m:{} d:{}", n, m, d);

    crate::glayout::print_js(pos, edges);
    */
    yew::start_app::<Model>()
}
