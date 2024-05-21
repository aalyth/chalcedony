
class Bueno:
    v: str
    w: str

    fn default() -> Bueno:
        return Bueno {v: "hello", w: "world"} 

class Calculator:
    a: uint
    b: uint 
    res: uint 

    bueno: Bueno

    fn example(a: uint, b: uint) -> uint:
        return a + b

    fn new(a: uint, b: uint, bueno: Bueno) -> Calculator:
        return Calculator {a, b, res: a + b, bueno: bueno} 

    fn new(a: uint, b: uint) -> Calculator:
        return Calculator {a, b, res: 0, bueno: Bueno::default()} 

    fn default() -> Calculator:
        return Calculator {a: 0, b: 0, res: 0, bueno: Bueno::default()}

    fn compute(self) -> Calculator:
        self.res = self.a + self.b
        return self
                
    fn display(self):
        print(self)

let example = Calculator {a: 1, b: 2, res: 15, bueno: Bueno {v: "hello", w: "world"}}
Calculator::new(5, 6).compute().display()
print(example.res)
example.compute()
print(example.res)
