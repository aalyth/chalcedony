
fn main() -> void:
    let i = 1

    while i <= 50:
        let ans = ''
        if i % 3 == 0:
            ans += 'FIZZ'
        if i % 5 == 0:
            ans += 'BUZZ'

        if ans != '':
            print(ans)

        if ans == '':
            print('' + i)
        i += 1

let i = 1
while i <= 50:
    let ans = ''
    if i % 3 == 0:
        ans += 'FIZZ'
    if i % 5 == 0:
        ans += 'BUZZ'

    if ans != '':
        print(ans)

    if ans == '':
        print('' + i)
    i += 1
