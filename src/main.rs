mod turtle;
use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();
    turtle.run(_STAR);
}

const _SQUARE: &str = "
TO SQUARE :LEN
    REPEAT 4 [
        FORWARD :LEN
        RIGHT 90
    ]
END
SQUARE 100
";

const _FERN: &str = "
BACK 100
TO FERN :SIZE :SIGN
    if :SIZE > 1 [
        FD :SIZE
        RT 70 * :SIGN FERN :SIZE * 0.5 :SIGN * -1 LT 70 * :SIGN
        FD :SIZE
        LT 70 * :SIGN fern :SIZE * 0.5 :SIGN RT 70 * :SIGN
        RT 7 * :SIGN fern :SIZE - 1 :SIGN LT 7 * :SIGN
        BK :SIZE * 2
    ]
END
FERN 25 1
";

const _TREE: &str = "
TO TREE :SIZE
    IF :SIZE > 5 [
        FD :SIZE / 3
        LT 30 TREE :SIZE * 0.66 RT 30
        FD :SIZE / 6
        RT 25 TREE :SIZE / 2 LT 25
        FD :size / 3
        RT 25 TREE :SIZE / 2 LT 25
        FD :SIZE / 6
        BK :SIZE
    ]
END
TREE 150
";

const _SNOWFLAKE: &str = "
to SIDE :LEN :DEP
    IF :DEP != 0 [
        SIDE :LEN / 3 :DEP - 1
        LT 60
        SIDE :LEN / 3 :DEP - 1
        RT 120
        SIDE :LEN / 3 :DEP - 1
        LT 60
        SIDE :LEN / 3 :DEP - 1
    ]
    IF :DEP == 0 [
        FD :LEN
    ]
END
TO SNOWFLAKE :LEN :DEP
    REPEAT 3 [
        SIDE :LEN :DEP
        RT 120
    ]
END
SNOWFLAKE 200 4
";

const _STAR: &str = "
BK 100
TO STAR :SIZE
    IF :SIZE >= 10 [
        REPEAT 5 [
            FD :SIZE
            STAR: :SIZE * 0.3
            RT 144
        ]
    ]
END
STAR 200
";