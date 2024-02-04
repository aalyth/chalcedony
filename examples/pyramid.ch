
fn left_triangle(n: int):
    let width = 1
    while n > 0:
        let i = 0
        let row = ''
        while i < width:
            row += '*'
            i += 1
        width += 1
        print(row)
        n -= 1
    print('')

fn pyramid(n: int):
    let width = n % 2
    let i = 0
    while i < n:
        let offset = 0
        let row = ''

        while offset < (n - width) / 2:
            row += ' ' 
            offset += 1

        let j = 0
        while j < width:
            row += '*'
            j += 1

        print(row)
        width += 2
        i += 2

left_triangle(5)
left_triangle(10)
pyramid(21)
