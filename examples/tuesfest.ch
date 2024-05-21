import "math.ch"

const pi_half = pi / 2

# overloaded alternative of `sin()`, rounding the result
fn sin(x: float, precision: uint) -> float:
    return round(sin(x), precision)

# overloaded alternative of `cos()`, rounding the result
fn cos(x: float, precision: uint) -> float:
    return round(cos(x), precision)

# example of an unsafe function, which throws an unguarded exception 
fn tg!(x: float) -> float:
    if x % pi_half == 0:
        throw "invalid x value: " + x 
    return round(sin(x) / cos(x), 3)


fn assert_rounded(lhs: float, rhs: float, n: uint):
    assert(round(lhs, n) == round(rhs, n))

if __name__ == '__main__':
    try:
        print(tg!(pi/6))
        print(tg!(pi/4))
        print(tg!(pi/3))
        print(tg!(pi/2))
    catch (exc: exception):
        print("Caught the exception: " + exc)

    let sqrt_3 = sqrt(3.0)
    assert_rounded(1.0 / sqrt_3, tg!(pi/6), 3)
    assert_rounded(1.0, tg!(pi/4), 3)
    assert_rounded(sqrt_3, tg!(pi/3), 3)
