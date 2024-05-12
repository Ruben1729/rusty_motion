use rusty_motion::prelude::*;

fn main() {
    let mut rm = rusty_motion::init()
        .size(640, 480)
        .title("Square Pan")
        .build();
    
    rm.render_single(Primitive::Square(10, Color::BLACK), 10)
                        .add_property(Property::Duration(10))
                        .add_property(Property::Direction(AnimDireciton::Alternate))
                        .add_property(Property::Iteration(AnimIteration::Infinite))
                        .add_keyframe(50, Move::Right(10))
                        .add_keyframe(100, Move::Right(200))
                        .build();
    
    rm.run();
}
