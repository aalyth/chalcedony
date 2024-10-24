#!/usr/local/bin/chal

fn test!():
    let a = "some example value"
    a += 698
    throw "bueno"
    print(a)

if __name__ == '__main__':
    try:
        test!()
    catch (exc: exception):
        print("Caught the exception: " + exc)
