
fn fib(n: int) -> int:
    if n > 2:
        return fib(n-1) + fib(n-2)
    return 1

fn test() -> void:
    return 191

fn main() -> void:
    test()
    let i = 35
    let res = fib(i)
    print('Fib ' + i + ': ' + res)
