
fn branch_test_1() -> str:
    if false:
        return 'one'
    elif false:
        return 'two'
    elif false: 
        return 'three'
    elif true: 
        return 'four'
    else:
        return 'five'
    return "this shouldn't be reached"

fn branch_test_2() -> str:
    if true:
        return 'one'
    elif false:
        return 'two'
    elif false: 
        return 'three'
    elif true: 
        return 'four'
    else:
        return 'five'
    return "this shouldn't be reached"

fn branch_test_3() -> str:
    if false:
        return 'one'
    elif false:
        return 'two'
    elif false: 
        return 'three'
    elif false: 
        return 'four'
    else:
        return 'five'
    return "this shouldn't be reached"

assert('four', branch_test_1())
assert('one', branch_test_2())
assert('five', branch_test_3())
