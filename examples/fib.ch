
fn fib(n: int) -> int:
    if n <= 2:
        return fib(n-1) + fib(n-2)
    return 1

fn main() -> void:
    print('Fib 5: ' + fib(5))
