
# esentially, the length of the digit
fn log10(n: uint) -> uint:
    if n == 0:
        return 1

    let res = 0
    while n > 0:
        res += 1
        n /= 10 

    return res

# gets the Nth digit (counts from 1 up)
# returns -1 on error
fn nth_digit(num: uint, n: uint) -> int:
    if n == 0:
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

    let i = 1
    while i <= len/2:
        let l = nth_digit(n, i)
        let r = nth_digit(n, len - i + 1)
        if l != r:
            return false
        i += 1

    return true

if __name__ == '__main__':
    assert(true  == is_palindrome(7))
    assert(false == is_palindrome(12))
    assert(true  == is_palindrome(11))
    assert(false == is_palindrome(123))
    assert(true  == is_palindrome(121))

    assert(false == is_palindrome(123421))
    assert(true  == is_palindrome(1337331))
