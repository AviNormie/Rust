use axum::Json;

#[derive(serde::Serialize)]

pub struct Vehicle {
    id: uuid::Uuid,
    make: String,
    model: String,
    year: u16,
}

pub async fn vehicle_get() -> Json<Vehicle> {
    Json::from(
    Vehicle {
        id: uuid::Uuid::new_v4(),
        make: "Toyota".to_string(),
        model: "Camry".to_string(),
        year: 2020,
    }) 
}

pub async fn vehicle_post(){

}
 