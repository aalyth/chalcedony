

fn fib_rec(n: int) -> int:
    if n > 2:
        return fib_rec(n-1) + fib_rec(n-2)
    return 1

fn fib(n: int) -> int:
    let a = 1
    let b = 1
    while n > 2:
        let c = a + b
        a = b
        b = c
        n -= 1
    return b

let i: int = 35
let res = fib(i)
print('Fib ' + i + ': ' + res)

assert(1, fib(2))
