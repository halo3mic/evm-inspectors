// use crate::utils::{write_traces, TestEvm};
// use alloy_primitives::{bytes, Bytes, U256};
// use alloy_sol_types::{sol, SolCall};
// use expect_test::expect;
// use revm_inspectors::tracing::{TracingInspector, TracingInspectorConfig};

// #[test]
// fn basic_trace_printing() {
//     // solc testdata/Counter.sol --via-ir --optimize --bin
//     sol!("testdata/Counter.sol");
//     static BYTECODE: Bytes = bytes!("60808060405234610016576102e2908161001b8239f35b5f80fdfe608060408181526004361015610013575f80fd5b5f915f3560e01c9081633fb5c1cb146102475781638381f58a1461022e57508063943ee48c146101885780639db265eb1461012f578063d09de08a146101105763f267ce9e14610061575f80fd5b346100ff57816003193601126100ff57610079610287565b303b156100ff578051639db265eb60e01b81528290818160048183305af18015610103576100eb575b5060607f4544f35949a681d9e47cca4aa47bb4add2aad7bf475fac397d0eddc4efe69eda91549268343490333937b6901960b91b8151916020835260096020840152820152a280f35b6100f49061025f565b6100ff57815f6100a2565b5080fd5b50505051903d90823e3d90fd5b823461012c578060031936011261012c57610129610287565b80f35b80fd5b50346100ff57816003193601126100ff577f4ada34a03bac92ee05461fb68ac194ed75b2b3ac9c428a50c1240505512954d560608354926868692066726f6d203360b81b8151916020835260096020840152820152a280f35b503461022a575f36600319011261022a575f547f4ada34a03bac92ee05461fb68ac194ed75b2b3ac9c428a50c1240505512954d56060835160208152600960208201526868692066726f6d203160b81b85820152a2303b1561022a578051637933e74f60e11b8152905f8260048183305af19081156102215750610210575b50610129610287565b61021a915061025f565b5f80610207565b513d5f823e3d90fd5b5f80fd5b3461022a575f36600319011261022a576020905f548152f35b3461022a57602036600319011261022a576004355f55005b67ffffffffffffffff811161027357604052565b634e487b7160e01b5f52604160045260245ffd5b5f545f198114610298576001015f55565b634e487b7160e01b5f52601160045260245ffdfea2646970667358221220e2a4410c976bdf76baab910915ab68a6487152ba1ea5836d41a16ac8042a36c864736f6c63430008180033");

//     let mut evm = TestEvm::new();

//     let mut tracer = TracingInspector::new(TracingInspectorConfig::all());
//     let address = evm.deploy(BYTECODE.clone(), &mut tracer).unwrap();
//     let mut s = write_traces(&tracer);
//     patch_output(&mut s);
//     expect![[r#"
//         . [147802] → new <unknown>@0xBd770416a3345F91E4B34576cb804a576fa48EB1
//             └─ ← [Return] 738 bytes of code
//     "#]]
//     .assert_eq(&s);

//     let mut call = |data: Vec<u8>| -> String {
//         let mut tracer = TracingInspector::new(TracingInspectorConfig::all());
//         let r = evm.call(address, data.into(), &mut tracer).unwrap();
//         assert!(r.is_success());
//         write_traces(&tracer)
//     };

//     let mut s = call(Counter::numberCall {}.abi_encode());
//     patch_output(&mut s);
//     expect![[r#"
//         . [2277] 0xBd770416a3345F91E4B34576cb804a576fa48EB1::8381f58a()
//             └─ ← [Return] 0x0000000000000000000000000000000000000000000000000000000000000000
//     "#]]
//     .assert_eq(&s);

//     let mut s = call(Counter::incrementCall {}.abi_encode());
//     patch_output(&mut s);
//     expect![[r#"
//         . [22390] 0xBd770416a3345F91E4B34576cb804a576fa48EB1::d09de08a()
//             └─ ← [Return] 
//     "#]]
//     .assert_eq(&s);

//     let mut s = call(Counter::numberCall {}.abi_encode());
//     patch_output(&mut s);
//     expect![[r#"
//         . [2277] 0xBd770416a3345F91E4B34576cb804a576fa48EB1::8381f58a()
//             └─ ← [Return] 0x0000000000000000000000000000000000000000000000000000000000000001
//     "#]]
//     .assert_eq(&s);

//     let mut s = call(Counter::setNumberCall { newNumber: U256::from(69) }.abi_encode());
//     patch_output(&mut s);
//     expect![[r#"
//         . [5144] 0xBd770416a3345F91E4B34576cb804a576fa48EB1::3fb5c1cb(0000000000000000000000000000000000000000000000000000000000000045)
//             └─ ← [Stop] 
//     "#]]
//     .assert_eq(&s);

//     let mut s = call(Counter::numberCall {}.abi_encode());
//     patch_output(&mut s);
//     expect![[r#"
//         . [2277] 0xBd770416a3345F91E4B34576cb804a576fa48EB1::8381f58a()
//             └─ ← [Return] 0x0000000000000000000000000000000000000000000000000000000000000045
//     "#]]
//     .assert_eq(&s);

//     let mut s = call(Counter::nest1Call {}.abi_encode());
//     patch_output(&mut s);
//     expect![[r#"
//         . [12917] 0xBd770416a3345F91E4B34576cb804a576fa48EB1::943ee48c()
//             ├─  emit topic 0: 0x4ada34a03bac92ee05461fb68ac194ed75b2b3ac9c428a50c1240505512954d5
//             │        topic 1: 0x0000000000000000000000000000000000000000000000000000000000000045
//             │           data: 0x0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000968692066726f6d20310000000000000000000000000000000000000000000000
//             ├─ [8035] 0xBd770416a3345F91E4B34576cb804a576fa48EB1::f267ce9e()
//             │   ├─ [2277] 0xBd770416a3345F91E4B34576cb804a576fa48EB1::9db265eb()
//             │   │   ├─  emit topic 0: 0x4ada34a03bac92ee05461fb68ac194ed75b2b3ac9c428a50c1240505512954d5
//             │   │   │        topic 1: 0x0000000000000000000000000000000000000000000000000000000000000046
//             │   │   │           data: 0x0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000968692066726f6d20330000000000000000000000000000000000000000000000
//             │   │   └─ ← [Return] 
//             │   ├─  emit topic 0: 0x4544f35949a681d9e47cca4aa47bb4add2aad7bf475fac397d0eddc4efe69eda
//             │   │        topic 1: 0x0000000000000000000000000000000000000000000000000000000000000046
//             │   │           data: 0x0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000968692066726f6d20320000000000000000000000000000000000000000000000
//             │   └─ ← [Return] 
//             └─ ← [Return] 
//     "#]]
//     .assert_eq(&s);
// }

// // Without this, `expect_test` fails on its own updated test output.
// fn patch_output(s: &mut str) {
//     (unsafe { s[0..1].as_bytes_mut() })[0] = b'.';
// }
