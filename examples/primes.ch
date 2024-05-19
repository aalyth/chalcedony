
fn is_prime(n: uint) -> bool:
    if n <= 1:
        return false 

    let i = 2 
    while i < n:
        if n % i == 0:
            return false 
        i += 1

    return true 

if __name__ == '__main__':
    let i = 0
    while i < 100:
        if is_prime(i):
            print("" + i + " is prime")
        i += 1

    assert(true, is_prime(2))
    assert(true, is_prime(3))
    assert(true, is_prime(17))
    assert(false, is_prime(42))

