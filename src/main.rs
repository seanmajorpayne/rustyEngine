use rusty_engine::run;

fn main() {
    pollster::block_on(run());
}