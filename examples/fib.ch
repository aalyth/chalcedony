

fn fib(n: int) -> int:
    if n > 2:
        return fib(n-1) + fib(n-2)
    return 1

fn test() -> int:
    return 1 * 2  

let i: int = 35
let res = fib(i)
print(i)
print('Fib ' + i + ': ' + res)
