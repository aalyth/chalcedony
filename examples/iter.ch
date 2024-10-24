#!/usr/local/bin/chal

class Range:
    current: int
    end: int
    step: int

    fn __iter__(self) -> Range:
        return self

    fn __next__!(self) -> int:
        if self.end > 0 && self.current >= self.end:
            throw "end iteration"
        elif self.end <= 0 && self.current <= self.end:
            throw "end iteration"

        self.current += self.step
        return self.current - self.step

fn range(end: int) -> Range:
    let step: int = 1
    if end < 0:
        step = -1
    return Range {current: 0, end, step}

fn range(start: int, end: int) -> Range:
    let step: int = 1 
    if start > end:
        step = -1
    return Range {current: start, end, step}

fn range(start: int, end: int, step: int) -> Range:
    return Range {current: start, end, step}

if __name__ == '__main__':
    print("Range to 5:")
    for i in range(5):
        print(i)

    print("Range in [4, 15):")
    for i in range(4, 15):
        print(i)

    print("Odd numbers to 16:")
    for i in range(1, 16, 2):
        print(i)
