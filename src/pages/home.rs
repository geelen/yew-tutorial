use styled_yew::styled;
use yew::prelude::*;

#[derive(Clone, Properties)]
struct DivProps {
    children: Children,
}
struct Div {
    props: DivProps,
}

impl Component for Div {
    type Message = ();
    type Properties = DivProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! { <div>{ self.props.children.render() }</div> }
    }
}

struct Product {
    id: i32,
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

styled!(Red : Div {
	color: "red";
});


impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let products: Vec<Product> = vec![
            Product {
                id: 1,
                name: "apple".to_string(),
                description: "an apple a day keeps the doctor away".to_string(),
                image: "https://source.unsplash.com/featured/300x300?apple".to_string(),
                price: 3.65,
            },
            Product {
                id: 2,
                name: "banana".to_string(),
                description: "an old banana leaf was once young and green".to_string(),
                image: "https://source.unsplash.com/featured/300x300?banana".to_string(),
                price: 7.99,
            },
        ];

        Self {
            state: State { products },
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let products: Vec<Html> = self
            .state
            .products
            .iter()
            .map(|product: &Product| {
                html! {
                  <Red>
                    <img src={&product.image}/>
                    <div>{&product.name}</div>
                    <div>{"$"}{&product.price}</div>
                  </Red>
                }
            })
            .collect();

        html! { <span>{products}</span> }
    }
}
