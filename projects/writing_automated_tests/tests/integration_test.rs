use writing_automated_tests::adder::adder;
use writing_automated_tests::rect::rect::Rectangle;

#[test]
fn integration_test() {
    assert_eq!(4, adder::add_two(2));

    let r = Rectangle {
        width: 2,
        height: (adder::add_two(2) as u32),
    };
    assert_eq!(4, r.get_height());
}
