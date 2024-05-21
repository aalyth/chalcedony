use chalcedony::common::Bytecode;
use chalcedony::interpreter::Chalcedony;
use chalcedony::vm::Cvm;

#[test]
fn valid_features_set() {
    if !cfg!(feature = "testing") {
        panic!("The feature `testing` must be set in order for tests to be valid.")
    }
}

// These 2 functions are crucial for testing since all CVM-based tests rely on
// the proper functioning `Bytecode::Assert` instruction.
#[test]
#[should_panic]
fn interpret_invalid_assert() {
    let mut vm = Cvm::new();
    let invalid_assert = vec![
        Bytecode::ConstU(12),
        Bytecode::ConstI(22),
        Bytecode::Eq,
        Bytecode::Assert,
    ];
    vm.execute(invalid_assert);
}

#[test]
fn interpret_valid_assert() {
    let mut vm = Cvm::new();
    let valid_assert = vec![
        Bytecode::ConstU(42),
        Bytecode::ConstU(42),
        Bytecode::Eq,
        Bytecode::Assert,
    ];
    let valid_assert2 = vec![
        Bytecode::ConstS("good".to_string().into()),
        Bytecode::ConstS("good".to_string().into()),
        Bytecode::Eq,
        Bytecode::Assert,
    ];
    vm.execute(valid_assert);
    vm.execute(valid_assert2);
}

#[test]
fn interpret_fibonacci() {
    let mut interpreter = Chalcedony::new();
    let fib_id = interpreter.get_next_func_id();

    let fib = vec![
        // fn fib(n: int) -> uint:
        Bytecode::CreateFunc(1),
        // if n > 2:
        Bytecode::GetLocal(0),
        Bytecode::ConstU(2),
        Bytecode::Gt,
        Bytecode::If(11),
        // return fib(n-2) + fib(n-1)
        Bytecode::GetLocal(0),
        Bytecode::ConstU(2),
        Bytecode::Sub,
        Bytecode::CallFunc(fib_id),
        Bytecode::GetLocal(0),
        Bytecode::ConstU(1),
        Bytecode::Sub,
        Bytecode::CallFunc(fib_id),
        Bytecode::Add,
        Bytecode::Return,
        Bytecode::Nop,
        // return 1
        Bytecode::ConstU(1),
        Bytecode::Return,
    ];

    /* assert that `fib(10) == 55` */
    let code = vec![
        Bytecode::ConstI(10),
        Bytecode::CallFunc(fib_id),
        Bytecode::ConstU(55),
        Bytecode::Eq,
        Bytecode::Assert,
    ];

    interpreter.execute(fib);
    interpreter.execute(code);
}

#[test]
fn interpret_guarded_exception() {
    let mut vm = Cvm::new();
    let code = vec![
        // try:
        Bytecode::TryScope(9),
        // print(21 * 2)
        Bytecode::ConstU(21),
        Bytecode::ConstU(2),
        Bytecode::Mul,
        Bytecode::Print,
        // throw "unexpected error"
        Bytecode::ConstS("unexpected error".to_string().into()),
        Bytecode::ThrowException,
        // print("all according to plan")
        Bytecode::ConstS("all according to plan".to_string().into()),
        Bytecode::Print,
        Bytecode::CatchJmp(5),
        // catch (exc: exception):
        //     print("Received the exception" + exc)
        Bytecode::ConstS("Received the exception: ".to_string().into()),
        Bytecode::GetLocal(0),
        Bytecode::Add,
        Bytecode::Print,
    ];

    /* this should not panic */
    vm.execute(code);
}

#[test]
#[should_panic]
fn interpret_unhandled_exception() {
    let mut vm = Cvm::new();

    let code = vec![
        // print(21 * 2)
        Bytecode::ConstU(21),
        Bytecode::ConstU(2),
        Bytecode::Mul,
        Bytecode::Print,
        // throw "unexpected error"
        Bytecode::ConstS("unexpected error".to_string().into()),
        Bytecode::ThrowException,
        // print("all according to plan")
        Bytecode::ConstS("all according to plan".to_string().into()),
        Bytecode::Print,
    ];
    vm.execute(code);
}

#[test]
fn interpret_instance_creation_and_access() {
    let mut vm = Cvm::new();

    let code = vec![
        // let a = Example {
        Bytecode::ConstObj(3),
        //     field1: 1,
        Bytecode::ConstU(1),
        Bytecode::SetAttr(0),
        //     field2: -10,
        Bytecode::ConstI(-10),
        Bytecode::SetAttr(1),
        //     words: OtherExample {
        //         hello: "hello ",
        //         world: "world"
        //     },
        Bytecode::ConstObj(2),
        Bytecode::ConstS("hello ".to_string().into()),
        Bytecode::SetAttr(0),
        Bytecode::ConstS("world".to_string().into()),
        Bytecode::SetAttr(1),
        Bytecode::SetAttr(2),
        Bytecode::SetLocal(0),
        // }
        // assert(-10, a.field2)
        Bytecode::ConstI(-10),
        Bytecode::GetLocal(0),
        Bytecode::GetAttr(1),
        Bytecode::Eq,
        Bytecode::Assert,
        // assert("hello world", a.words.hello + a.words.world)
        Bytecode::ConstS("hello world".to_string().into()),
        Bytecode::GetLocal(0),
        Bytecode::GetAttr(2),
        Bytecode::GetAttr(0),
        Bytecode::GetLocal(0),
        Bytecode::GetAttr(2),
        Bytecode::GetAttr(1),
        Bytecode::Add,
        Bytecode::Eq,
        Bytecode::Assert,
    ];
    vm.execute(code);
}
