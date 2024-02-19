
fn fizzbuzz(n: uint) -> str:
    let res = ""

    if n % 3 == 0:
        res += "fizz"

    if n % 5 == 0:
        res += "buzz"

    if res != "":
        return res 

    return  "" + n

fn __test__() -> void:
    assert("fizz", fizzbuzz(9))
    assert("buzz", fizzbuzz(10))
    assert("fizzbuzz", fizzbuzz(75))
    assert("47", fizzbuzz(47))

let i = 1
while i <= 30:
    print(fizzbuzz(i))
    i += 1

__test__()
