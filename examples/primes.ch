
fn is_prime(n: int) -> bool:
    if n == 0:
        return false 

    let i = 2 
    while i < n:
        if n % i == 0:
            return false 
        i += 1

    return true 

fn __main__() -> void:
    let i = 0
    let sum = 0
    while i < 10000:
        if is_prime(i):
            sum += i
        i += 1
    print(sum)

__main__()
