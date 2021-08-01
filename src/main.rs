use yew::{html, Component, ComponentLink, Html, ShouldRender};

mod glayout;

struct GraphConfig {
    n_vertices: usize,
    n_dimensions: usize,
    max_degree: usize,
}

struct LayoutConfig {
    n_iters: usize,
}

enum Msg {
    NewGraph(GraphConfig),
    Layout(LayoutConfig),
}

struct Model {
    link: ComponentLink<Self>,
    vertices: Vec<Vec<f32>>,
    edges: Vec<usize>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            vertices: Vec::new(),
            edges: Vec::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::NewGraph(GraphConfig {
                n_vertices,
                n_dimensions,
                max_degree,
            }) => true,
            Msg::Layout(LayoutConfig { n_iters }) => true,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>

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
