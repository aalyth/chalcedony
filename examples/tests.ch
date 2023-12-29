# let a := 1 + 2 * 3 - 4 / -5
# let b := !(5 - a)
# 
# let d := -5.2*--3
# let e := fib(min(2 + 3 * 4, 5 + 7 * 6 / 3), 2 * 3 / 2) + fib( min(5, 6) - 2 ) * 2
# let f := -(-(-5))
# 
# let g := fib(-min(2 + 3 * 4, - 5 + 7 * 6 / 3), - 2 * 3 / 2) + fib( min(5, 6) - 2 ) * 2
# let h := 2 || 3 + !(12 / 4 * 2)

let c = 34.5*(23+1.0)/2

fn is_prime(n: int) -> bool:
    if n == 0:
        return false 

    let i = 2 
    while i < n:
        if n % i == 0:
            return false 
        i += 1

    return true 

fn sum(n: int) -> int:
    if n > 1:
        return n + sum(n-1)
    return 1

fn main() -> void:
    print('' + c)
    let i = 0
    let sum = 0
    while i < 5000:
        if is_prime(i):
            sum += i
        i += 1
    print('' + sum)
