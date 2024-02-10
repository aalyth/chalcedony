

fn fib_rec(n: int) -> int:
    if n > 2:
        return fib_rec(n-1) + fib_rec(n-2)
    return 1

fn fib(n: int) -> int:
    let a = 1
    let b = 1
    while n >= 0:
        let c = a + b
        a = b
        b = c
        n -= 1
    return b

let i: int = 10 
let res = fib(i)
let res_rec = fib_rec(i)
print('Fib (' + i + '): ' + res + '; exp: ' + res_rec)

assert(fib(i), fib_rec(i))
