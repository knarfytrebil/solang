use parity_scale_codec::Encode;
use parity_scale_codec_derive::{Decode, Encode};

use super::{build_solidity, first_error, no_errors};
use solang::{parse_and_resolve, Target};

#[test]
fn weekdays() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Val(u8);

    // parse
    let (runtime, mut store) = build_solidity(
        "
        contract enum_example {
            enum Weekday { Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday }

            function is_weekend(Weekday day) public pure returns (bool) {
                return (day == Weekday.Saturday || day == Weekday.Sunday);
            }

            function test_values() public pure {
                assert(int8(Weekday.Monday) == 0);
                assert(int8(Weekday.Tuesday) == 1);
                assert(int8(Weekday.Wednesday) == 2);
                assert(int8(Weekday.Thursday) == 3);
                assert(int8(Weekday.Friday) == 4);
                assert(int8(Weekday.Saturday) == 5);
                assert(int8(Weekday.Sunday) == 6);

                Weekday x;

                assert(uint(x) == 0);

                x = Weekday.Sunday;
                assert(int16(x) == 6);

                x = Weekday(2);
                assert(x == Weekday.Wednesday);
            }
        }",
    );

    runtime.function(&mut store, "is_weekend", Val(4).encode());

    assert_eq!(store.scratch, Val(0).encode());

    runtime.function(&mut store, "is_weekend", Val(5).encode());

    assert_eq!(store.scratch, Val(1).encode());

    runtime.function(&mut store, "test_values", Vec::new());
}

#[test]
fn test_cast_errors() {
    let (_, errors) = parse_and_resolve(
        "contract test {
            enum state { foo, bar, baz }
            function foo() public pure returns (uint8) {
                return state.foo;
            }
        }",
        &Target::Substrate,
    );

    assert_eq!(
        first_error(errors),
        "conversion from enum test.state to uint8 not possible"
    );

    let (_, errors) = parse_and_resolve(
        "contract test {
            enum state {  }
            function foo() public pure returns (uint8) {
                return state.foo;
            }
        }",
        &Target::Substrate,
    );

    assert_eq!(first_error(errors), "enum ‘state’ is missing fields");

    let (_, errors) = parse_and_resolve(
        "contract test {
            enum state { foo, bar, baz }
            function foo() public pure returns (uint8) {
                return uint8(state.foo);
            }
        }",
        &Target::Substrate,
    );

    no_errors(errors);
}
