use criterion::{criterion_group, criterion_main, Criterion};

fn moves(c: &mut Criterion) {
    c.bench_function("initial positions", |b| {
        b.iter(move || {
            let game = checkers::Game{
                side: checkers::Player::BLK,
                blk: checkers::Board::new(0b0000_0000_0000_0000_0000_1111_1111_1111),
                red: checkers::Board::new(0b1111_1111_1111_0000_0000_0000_0000_0000),
                king: checkers::Board::new(0b0000_0000_0000_0000_0000_0000_0000_0000),
            };

            let _v: Vec<checkers::Move> = game.moves().collect();
        });
    });

    c.bench_function("complex positions", |b| {
        b.iter(move || {
            let game = checkers::Game{
                side: checkers::Player::BLK,
                blk: checkers::Board::new(0b0000_0010_0000_0100_0000_0000_1111_0000),
                red: checkers::Board::new(0b1010_0000_0101_0000_1010_0000_0000_0000),
                king: checkers::Board::new(0b0010_0000_0001_0000_0000_0000_0010_0000),
            };

            let _v: Vec<checkers::Move> = game.moves().collect();
        });
    });
}

criterion_group!(benches, moves);
criterion_main!(benches);
