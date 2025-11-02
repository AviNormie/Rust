
use axum::{ Router, routing::get};

mod vehicle;

#[tokio::main]

async fn main() {
    println!("Hello, world!");
   let router1 = Router::new().route("/vehicle", get(vehicle::vehicle_get).post(vehicle::vehicle_post));
   // defining port listener. 
   let address = "0.0.0.0:8000";
   let listener = tokio::net::TcpListener::bind(address).await.unwrap();

   // serving to launch

   axum::serve(listener, router1).await.unwrap();

}
