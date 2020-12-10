mod turtle;
use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();
    turtle.run(_FERN);
}

const _TEST: &str = "
forward 200 * 0.5
right 45 * 2
forward 50 * 2.0
";

const _TEST2: &str = "
to square :first :second
    repeat 4 [
        forward :first
        right 90
    ]    
    if :second > 50 [  
        square :second 0
    ]
end
square 100 150
";

/*
[
    To(
        "square",
        [":first", ":second"],
        [
            If(
                Condition(Var(":second"), Gtr, Number(50.0)),
                [
                    Repeat(
                        Number(4.0),
                        [
                            Forward(Var(":first")),
                            Right(Number(90.0))
                        ]
                    ),
                    Call("square", [Var(":second"), Number(0.0)])
                ]
            )
        ]
    ),
    Call("square", [Number(100.0), Number(150.0)])]
*/

const _FORWARD: &str = "
forward 100
";

const _ANGLE: &str = "
right 30 forward 100
right 90 forward 100
";

const _REPEAT: &str = "
repeat 4 [
    forward 100 right 90
]
";

const _TO_SQUARE: &str = "
to square :len
    repeat 4 [
        forward :len
        right 90
    ]
end
square 50
square 100
";

const _TO_SQUARE_IF: &str = "
to square :len
    if 50 < :len [
        repeat 4 [
            forward :len
            right 90
        ]
    ]
end
square 50
square 100
";

const _TO_RECT: &str = "
to rect :len
    repeat 2 [
        forward :len
        right 90
        forward :len * 2
        right 90
    ]
end
rect 100-10
";

const _SQUARE_FROM_RECT: &str = "
to rect :a :b
    repeat 2 [
        forward :a
        right 90
        forward :b
        right 90
    ]
end
to square :a
    rect :a :a
end
rect 100 150
square 50
";

const _A: &str = "
to a :len
    if :len > 2 [
        forward :len
        right 30
        a :len * 0.5
    ]
end
a 100
";

const _FERN: &str = "
to fern :size :sign
    if :size > 1 [
        forward :size
        right 70 * :sign fern :size * 0.5 :sign * -1 left 70 * :sign
        forward :size
        left 70 * :sign fern :size * 0.5 :sign right 70 * :sign
        right 7 * :sign fern :size - 1 :sign left 7 * :sign
        backward :size * 2
    ]
end
fern 25 1
";

/*
[
    To(
        "square",
        [":len"],
        [
            Repeat(
                Number(4.0),
                [
                    Forward(Var(":len")),
                    Right(Number(90.0))
                ]
            )
        ]
    ),
    Call("square", [Number(50.0)]),
    Call("square", [Number(100.0)])
]
*/


/*
[
    To(
        "a",
        [":len"],
        [
            If(
                Condition(Var(":len"), Gtr, Number(0.0)),
                [
                    Forward(Var(":len")),
                    Right(Number(30.0)),
                    Call("a", [Math(Var(":len"), Sub, Number(1.0))])
                ]
            )
        ]
    ),
    Call("a", [Number(5.0)])]
*/

/*
[
    To(
        "a",
        [":len"],
        [
            If(
                Condition(Var(":len"),Gtr, Number(0.0)),
                [
                    Forward(Var(":len")),
                    Right(Number(30.0)),
                    Call("a", [Math(Var(":len"), Sub, Number(1.0))])
                ]
            )
        ]
    ),
    Call("a", [Number(5.0)])]
*/