

fn fib_rec(n: int) -> int:
    if n > 2:
        return fib_rec(n-1) + fib_rec(n-2)
    return 1

fn fib(n: int) -> int:
    let a = 0
    let b = 1
    while n >= 2:
        let c = a + b
        a = b
        b = c
        n -= 1
    return b

if __name__ == '__main__':
    let n = 1
    while n <= 15:
        let res = fib(n)
        let res_rec = fib_rec(n)

        print('Fib (' + n + '): ' + res + '; exp: ' + res_rec)
        assert(res == res_rec)

        n += 1
