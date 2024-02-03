
fn is_prime(n: int) -> bool:
    print('n: ' + n)
    if n == 0:
        return false 

    let i = 2 
    while i < n:
        if n % i == 0:
            return false 
        i += 1

    return true 

fn main() -> void:
    let i = 0
    let sum = 0
    print('i: ' + i)
    print('' + is_prime(15))
    # while i < 1:
    #     if is_prime(i):
    #         sum += i
    #     i += 1
    # print('' + sum)

is_prime(12)
