
const pi = 3.141_592_653_589_793

fn gcd(a: uint, b: uint) -> uint:
    if b == 0:
        return a
    return gcd(b, a % b)

fn lcm(a: uint, b: uint) -> uint:
    return (a * b) / gcd(a, b)

# returns x to the power of y
fn pow(x: float, y: uint) -> float:
    if y == 0:
        return 1.0

    let result = 1.0
    while y > 0: 
        result *= x
        y -= 1
    return result

fn abs(x: float) -> float:
    if x < 0:
        return -x 
    return x

fn sgn(x: float) -> float:
    if x == 0.0:
        return 0.0
    elif x < 0:
        return -1.0
    return 1.0

fn round(x: float, n: uint) -> float:
    let modifier = pow(10.0, n)
    let int_repr = 1 * abs(modifier * 10.0 * x)
    if int_repr % 10 > 4:
        int_repr += 10
    int_repr /= 10
    return sgn(x) * int_repr / modifier

fn sin(x: float) -> float:
    x %= pi
    return x - (pow(x, 3) / 6) + (pow(x, 5) / 120) - (pow(x, 7) / 5040) + (pow(x, 9) / 362_880)

fn cos(x: float) -> float:
    x %= pi
    return 1.0 - (pow(x, 2) / 2) + (pow(x, 4) / 24) - (pow(x, 6) / 720) + (pow(x, 8) / 40_320)

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

fn __test_sine__():
    assert(0.0,    sin(0.0))
    assert(0.5,    round(sin(pi/6), 3))
    assert(0.707,  round(sin(pi/4), 3))
    assert(0.866,  round(sin(pi/3), 3))
    assert(1.0,    round(sin(pi/2), 3))

    assert(0.0,    sin(pi))
    assert(-0.5,   round(sin(-pi/6), 3))
    assert(-0.707, round(sin(-pi/4), 3))
    assert(-0.866, round(sin(-pi/3), 3))
    assert(-1.0,   round(sin(-pi/2), 3))

fn __test_cosine__():
    assert(1.0,    cos(0.0))
    assert(0.866,  round(cos(pi/6), 3))
    assert(0.707,  round(cos(pi/4), 3))
    assert(0.5,    round(cos(pi/3), 3))
    assert(0.0,    round(cos(pi/2), 3))

    assert(1.0,    cos(pi))
    assert(0.866,  round(cos(-pi/6), 3))
    assert(0.707,  round(cos(-pi/4), 3))
    assert(0.5,    round(cos(-pi/3), 3))
    assert(0.0,    round(cos(-pi/2), 3))

if __name__ == '__main__':
    assert(15, gcd(30, 75))
    assert(21, lcm(3, 7))
    __test_sine__()
    __test_cosine__()
    assert(1.414_213_562_373_095, sqrt(2.0))
