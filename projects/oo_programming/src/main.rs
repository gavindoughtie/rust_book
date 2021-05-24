use oo_programming::{AveragedCollection, Button, List, Screen};

fn main() {
    let mut ac = AveragedCollection::new();
    ac.add(100);
    ac.add(200);
    println!("average: {}", ac.average());

    let b1 = Box::new(Button{label: String::from("b1"), height: 20, width: 100});
    let b2 = Box::new(Button{label: String::from("b2"), height: 20, width: 100});
    let l1 = Box::new(List{items: vec![String::from("item1")], height: 200, width: 100});
    let screen = Screen{components: vec![b1, l1, b2]};
    screen.run();
}
