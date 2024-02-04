
let a = 15

fn stateful_example() -> void:
    let a = a
    a *= 2
    print(a + 2)

stateful_example()
a += 3
stateful_example()
