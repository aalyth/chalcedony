
# esentially, the length of the digit
fn log10(n: uint) -> uint:
    let res = 0
    while n > 0:
        res += 1
        n /= 10 

    return res

# gets the Nth digit (counts from 1 up)
# returns -1 on error
fn nth_digit(num: uint, n: uint) -> int:
    if n <= 0:
        return -1

    let len = log10(num)

    if n > len:
        return -1

    let res = 0
    while len - n > 0:
        num /= 10
        n += 1

    return num % 10

fn is_palindrome(n: uint) -> bool:
    let len = log10(n)

    if len % 2 == 1:
        len += 1

    let i = 1
    while i < len/2:
        let l = nth_digit(n, i)
        let r = nth_digit(n, len - i)
        print('l: ' + l)
        print('r: ' + r)
        if l != r:
            return false

        i += 1

    return true


let num = 519197
print(log10(num))
print(nth_digit(num, 7))
print(is_palindrome(12))
