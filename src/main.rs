mod turtle;
use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();
    turtle.run(LINE_CODE);
}

const _SQUARE_CODE: &str ="
to rect :arg1 :arg2
    if :arg1 * 2 > :arg2 [
        repeat 2 [
            forward :arg1
            right 90
            forward :arg2
            right 90
        ]
    ]
end
rect 10 20
";

const LINE_CODE: &str = "
forward 100
";

/*
[
    To(
        Ident("rect"),
        [Var(":arg1"), Var(":arg2")],
        [
            If(
                Condition(
                    Math(Var(":arg1"), Mul, Number(2)),
                    Gtr,
                    Var(":arg2")
                ),
                [
                    Repeat(
                        Number(2),
                        [
                            Forward(Var(":arg1")),
                            Right(Number(90)),
                            Forward(Var(":arg2")),
                            Right(Number(90))
                        ]
                    )
                ]
            )
        ]
    ),
    Call(Ident("rect"), [Number(10), Number(20)])]
*/