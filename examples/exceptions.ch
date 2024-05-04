
fn test!():
    let a = "some example value"
    a += 698
    throw "bueno"
    print(a)

try:
    test!()
catch (exc: exception):
    print("Caught the exception: " + exc)
