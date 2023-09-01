enum TrafficLight {
    Green,
    Red,
    Yellow,
}

trait TrafficLightSignal {
    fn time(&self) ->u32;
}

impl TrafficLightSignal for TrafficLight {
    fn time(&self)-> u32{
        match self {
           TrafficLight::Green => 15,
           TrafficLight::Red => 20,
           TrafficLight::Yellow => 3,
        }
    }
}


fn main(){
    let green_light = TrafficLight::Green;
    println!("Green light wait {:?} seconds",green_light.time());
    let red_light = TrafficLight::Red;
    println!("Red light wait {:?} seconds",red_light.time());
    let yellow_light = TrafficLight::Yellow;
    println!("Yellow light wait {:?} seconds ",yellow_light.time());
}
