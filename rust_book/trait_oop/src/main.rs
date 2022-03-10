pub trait Component {
    fn draw(&self);
    fn modify(&mut self, val: i32);
}

pub struct Square {
    edge: i32,
}

pub struct Circle {
    radius: i32,
}

impl Component for Square {
    fn draw(&self) {
        println!("Draw a square with edge {}", self.edge);
    }

    fn modify(&mut self, val: i32) {
        self.edge = val;
        println!("Change edge to {}", self.edge);
    }
}

impl Component for Circle {
    fn draw(&self) {
        println!("Draw a circle with radius {}", self.radius);
    }

    fn modify(&mut self, val: i32) {
        self.radius = val;
        println!("Change radius to {}", self.radius);
    }
}

fn main() {
    let mut components: Vec<Box<dyn Component>> = Vec::new();
    components.push(Box::new(Square { edge: 2 }));
    components.push(Box::new(Circle { radius: 2 }));

    for comp in components.iter_mut() {
        comp.modify(4);
    }
}
