
let pi = 3.14

fn circle_circ(radius: float) -> float:
    return 2.0 * pi * radius

assert(12.56, circle_circ(2.0)) 
assert(43.96, circle_circ(7.0)) 

assert(14.7266, circle_circ(2.345))

# this wrong approximation is only for testing purposes
pi = 3.0

# the result of the function depends on the external state, 
# but it cannot modify it from the inside
assert(27.9, circle_circ(4.65))
assert(14.07, circle_circ(2.345))


