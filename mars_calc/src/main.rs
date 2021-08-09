fn main() { // first call - entry point
    println!("Weight on Mark: {}kg", calculate_weight_on_mars(100.0) );//macro
}

fn calculate_weight_on_mars(weight: f32) -> f32{
    (weight / 9.81) * 3.711
}

