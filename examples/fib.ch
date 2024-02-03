

fn fib(n: int) -> int:
    if n > 2:
        return fib(n-1) + fib(n-2)
    return 1

let i: int = 35
let res = fib(i)
print('Fib ' + i + ': ' + res)
