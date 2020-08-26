use yew::prelude::*;

struct Product {
    name: String,
    description: String,
    image: String,
    price: f64,
}

struct State {
    products: Vec<Product>,
}

pub struct Home {
    state: State,
}

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            state: State { products: vec![] },
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! { <span>{"Home Sweet Home!"}</span> }
    }
}
