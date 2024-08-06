use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use product_api::{
    db::DB,
    model::Product,
    response::{ProductList, ProductRespose},
    ProductFilterParams,
};

#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    dotenv::dotenv().ok();
    env_logger::init();
    let state = Arc::new(AppState { db: DB::new() });
    let server_addr = option_env!("SERVER_ADDR").unwrap_or("127.0.0.1:3030");
    // build our application with a route
    let app = Router::new()
        .route("/products", get(products))
        .with_state(state);

    // run it
    let listener = tokio::net::TcpListener::bind(server_addr).await?;
    log::info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await
}
#[derive(Debug)]
struct AppState {
    db: DB,
}
async fn products(
    State(state): State<Arc<AppState>>,
    params: Query<ProductFilterParams>,
) -> Result<Json<ProductList>, StatusCode> {
    let products: Vec<&Product> = state
        .db
        .get_products()
        .iter()
        .filter(|x| x.category.as_str() == params.category)
        .filter(|x| match params.price_less_then {
            Some(price) => x.price <= price,
            None => true,
        })
        .collect();
    if products.len() == 0 {
        return Ok(Json(ProductList::default()));
    }
    let page_size = params.page_size.unwrap_or(5).min(products.len());
    let total_pages = products.len() / page_size;
    let page_num = params.page_num.unwrap_or(1);
    if total_pages < page_num {
        return Err(StatusCode::NOT_FOUND);
    }
    let start_rage = (page_num - 1) * page_size;
    let end_rage = page_num * page_size;
    let discounts = state.db.get_discounts();
    let mut out = vec![];
    for p in &products[start_rage..end_rage] {
        let price = (*p, &discounts[..]).into();
        out.push(ProductRespose {
            sku: p.sku.clone(),
            name: p.name.clone(),
            category: p.category.clone(),
            price,
        });
    }
    Ok(Json(ProductList {
        products: out,
        pagination: (page_num, page_size).into(),
    }))
}
