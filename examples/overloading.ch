
let pi = 3.141_592_653_589_793

# hello_world()
fn func(name: str) -> void:
    print("Hi, " + name)

# fib()
fn func(n: uint) -> uint:
    if n > 2:
        return func(n - 1) + func(n - 2)
    return 1

# pow()
fn func(x: float, y: uint) -> float:
    if y == 0:
        return 1
    let result = 1.0
    while y > 0: 
        result *= x
        y -= 1
    return result

# sin()
fn func(x: float) -> float:
    return x - (func(x, 3) / 6) + (func(x, 5) / 120) - (func(x, 7) / 5040) + (func(x, 9) / 362_880)


if __name__ == '__main__':
    # print()
    func("Cyberia")
    # fib()
    assert(55, func(10))
    # sin()
    assert(0.8660254450997811, func(pi/3))


