use oo_programming::{AveragedCollection, Button, List, Screen};
use blog::Post;

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

    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    // assert_eq!("", post.content());

    let post = post.request_review();
    // assert_eq!("", post.content());

    let post = post.approve();
    // post.content isn't available until you have two approvals
    let post = post.reject();
    let post = post.approve();
    // so this won't compile either!
    // println!("{}", post.content);

    let post = post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
    println!("post content: {}", post.content());
}
