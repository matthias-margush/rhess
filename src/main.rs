extern crate rhess;
//use rhess::game;
use std::iter;
use rhess::moves::*;
use rhess::pieces::Position;

fn main() {
    //println!("{:?}", game::game());

    let position = Position(3, 3);
    //let moves: Vec<Position> = position.movement(1, 1).take(4).collect();
    let moves = possible_bishop_moves(position);


    println!("Moves: {:?}", moves);

    let (i, j): (i32, i32) = (2, 3);

    let is = iter::repeat(i);
    let js = j..8;

    let foo: Vec<(i32,i32)> = is.zip(js).collect();

    println!("foo: {:?}", foo);

//    let r = [1, 2, 3];
//    let re: Vec<integer> = r.iter().collect();

    //let re: Vec<i32> = a.iter().map(|x| { x + 1 }).take_while(|x| { *x > -1 } ).collect();
//    let i = 0usize;
//    let r = [0; 8];
//    let re: Vec<Position> = (i..r.len()).rev().map(|x| { (x, i) }).take_while(on_board).collect();
//    println!("foo: {:?}", re);
}
