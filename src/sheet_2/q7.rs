enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn next_color(tl: &TrafficLight) -> TrafficLight {
    match tl {
        TrafficLight::Red => TrafficLight::Yellow,
        TrafficLight::Yellow => TrafficLight::Green,
        TrafficLight::Green => TrafficLight::Red,
    }
}
