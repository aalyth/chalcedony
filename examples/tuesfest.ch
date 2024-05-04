# x ^ y  
fn pow(x: float, y: uint) -> float:
    if y == 0:
        return 1.0

    let result = 1.0
    while y > 0: 
        result *= x
        y -= 1
    return result

# utilities 
fn sgn(x: float) -> float:
    if x == 0.0:
        return 0.0
    elif x < 0:
        return -1.0
    return 1.0

fn abs(x: float) -> float:
    if x < 0:
        return -x 
    return x

fn round(x: float, n: uint) -> float:
    let modifier = pow(10.0, n)
    let int_repr = 1 * abs(modifier * 10.0 * x)
    if int_repr % 10 > 4:
        int_repr += 10
    int_repr /= 10
    return sgn(x) * int_repr / modifier

let pi = 3.141_592_653_589_793
let pi_half = pi / 2

fn sin(x: float) -> float:
    x %= pi
    return x - (pow(x, 3) / 6) + (pow(x, 5) / 120) - (pow(x, 7) / 5040) + (pow(x, 9) / 362_880)

# overloaded alternative of `sin()`, rounding the result
fn sin(x: float, precision: uint) -> float:
    return round(sin(x), precision)

fn cos(x: float) -> float:
    x %= pi
    return 1.0 - (pow(x, 2) / 2) + (pow(x, 4) / 24) - (pow(x, 6) / 720) + (pow(x, 8) / 40_320)

# overloaded alternative of `cos()`, rounding the result
fn cos(x: float, precision: uint) -> float:
    return round(cos(x), precision)

# example of an unsafe function, which throws an unguarded exception 
fn tg!(x: float) -> float:
    if x % pi_half == 0:
        throw "invalid x value: " + x 
    return round(sin(x) / cos(x), 3)

# x is the estimate of the square root of S
fn __heron_sqrt__(S: float, x: float) -> float:
    return 0.5 * (x + S/x) 

fn sqrt(s: float) -> float:
    let x: float = s / 3
    let i = 0
    while i < 10:
        x = __heron_sqrt__(s, x) 
        i += 1
    return x

try:
    print(tg!(pi/6))
    print(tg!(pi/4))
    print(tg!(pi/3))
    print(tg!(pi/2))
catch (exc: exception):
    print("Caught the exception: " + exc)

fn assert_rounded(lhs: float, rhs: float, n: uint):
    assert(round(lhs, n), round(rhs, n))

let sqrt_3 = sqrt(3.0)
assert_rounded(1.0 / sqrt_3, tg!(pi/6), 3)
assert_rounded(1.0, tg!(pi/4), 3)
assert_rounded(sqrt_3, tg!(pi/3), 3)

