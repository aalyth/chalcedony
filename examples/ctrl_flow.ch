
fn test():
    let a = 0
    while a < 100:
        a += 1

        if a == 42:
            break

        elif a % 2 == 0:
            continue

        let j = 0
        while j < 10:
            let c = 0
            while c < 3:
                if c != 1:
                    c += 1
                    continue
                c += 1
                print("         c value - " + c)
            j += 1
            if j % 2 == 1:
                continue
            print("    j is even: " + j)
        print("odd - " + a)

test()
