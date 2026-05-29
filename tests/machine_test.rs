mod common;

use rust_cesk::conf::ProgramHandler;
use rust_cesk::conf::conf::Config;

#[test]
#[should_panic(expected = "exited with code IntV(1)")]
fn runs_basic_program_fixture() {
    let mut program = common::parse_fixture("basic_program.carb");
    let entry = program.get_entry().unwrap();
    let mut handler = ProgramHandler::from(program);
    let mut config = Config::from(&entry);

    for _ in 0..20 {
        config = config.next(&mut handler);
    }
}
