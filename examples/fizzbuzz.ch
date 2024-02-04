
fn __main__() -> void:
    let i = 1
    
    while i <= 50:
        let ans = ''
        if i % 3 == 0:
            ans += 'fizz'
        if i % 5 == 0:
            ans += 'buzz'
        
        if ans != '':
            print(ans)
        else:
            print(i)
        i += 1

__main__()
