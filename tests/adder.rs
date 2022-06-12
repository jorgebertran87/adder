use std::{convert::Infallible};

use async_trait::async_trait;
use cucumber::{given, then, when, World, WorldInit};

#[derive(Debug, WorldInit)]
struct Adder {
    number1: usize,
    number2: usize,
    result: usize,
}

#[async_trait(?Send)]
impl World for Adder {
    type Error = Infallible;

    async fn new() -> Result<Self, Self::Error> {
        Ok(Self { number1: 0, number2: 0, result: 0 })
    }
}

#[given(regex = r"^(\d+) and (\d+)$")]
async fn save_numbers(adder: &mut Adder, number1: usize, number2: usize) {
    adder.number1 = number1;
    adder.number2 = number2;
}

#[when("we add them")]
async fn add(adder: &mut Adder) {
    adder.result = adder.number1 + adder.number2;
}

#[then(regex = r"^the result is (\d+)$")]
async fn the_result_is(adder: &mut Adder, result: usize) {
    assert_eq!(result, adder.result, "{} is not correct", result);
}

fn main() {
    futures::executor::block_on(Adder::run("tests/features"));
}