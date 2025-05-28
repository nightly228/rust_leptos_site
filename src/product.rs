use leptos::*;

#[derive(Clone)]
pub struct Product {
    pub name: &'static str,
    pub description: &'static str,
    pub price: f64,
    pub image: &'static str,
}

impl Product {
    pub fn new(name: &'static str, description: &'static str, price: f64, image: &'static str) -> Self {
        Self { name, description, price, image }
    }
}

#[component]
pub fn ProductCard(product: Product) -> impl IntoView {
    view! {
        <div class="product-card">
            <img src={product.image} alt={product.name} />
            <h3>{product.name}</h3>
            <p class="price">{format!("{} ₽", product.price)}</p>
            <p>{product.description}</p>
        </div>
    }
}
