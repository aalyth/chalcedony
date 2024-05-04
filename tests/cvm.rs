use chalcedony::common::Bytecode;
use chalcedony::interpreter::Chalcedony;
use chalcedony::vm::Cvm;

#[test]
#[should_panic]
fn interpret_assert() {
    let mut vm = Cvm::new();
    let valid_assert = vec![Bytecode::ConstU(42), Bytecode::ConstU(42), Bytecode::Assert];
    let valid_assert2 = vec![
        Bytecode::ConstS("good".to_string().into()),
        Bytecode::ConstS("good".to_string().into()),
        Bytecode::Assert,
    ];
    let invalid_assert = vec![Bytecode::ConstU(12), Bytecode::ConstI(22), Bytecode::Assert];

    vm.execute(valid_assert);
    vm.execute(valid_assert2);
    vm.execute(invalid_assert);
}

#[test]
fn interpret_fibonacci() {
    let mut interpreter = Chalcedony::new();
    let fib_id = interpreter.get_next_func_id();

    let fib = vec![
        // fn fib(n: int) -> uint:
        Bytecode::CreateFunc(1),
        // if n > 2:
        Bytecode::GetArg(0),
        Bytecode::ConstU(2),
        Bytecode::Gt,
        Bytecode::If(11),
        // return fib(n-2) + fib(n-1)
        Bytecode::GetArg(0),
        Bytecode::ConstU(2),
        Bytecode::Sub,
        Bytecode::CallFunc(fib_id),
        Bytecode::GetArg(0),
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
        Bytecode::Assert,
    ];

    interpreter.execute(fib);
    interpreter.execute(code);
}
