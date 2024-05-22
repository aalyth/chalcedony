# Chalcedony

A Python-like scripting language and interpreter with a built-in static type checker. This is my diploma project for my Elsys graduation @2024.

For examples refer to [`/examples`](https://github.com/aalyth/Chalcedony/tree/main/examples).

# The idea

The idea behind this language is the increasing popularity of type systems for interpreted languages. With upcoming technologies and updates such as Typescript, PEP 484 (Type hints for Python), and RBS (Type signatures for Ruby) the concept of a language with these features built-in becomes more apparent.

Chalcedony also borrows ideas from other languages such as Rust or Go, aiming to solve problems such as unexpectedly raising exceptions, or mistakenly passing the wrong argument type to a function.

# Features

The language currently supports the following features:

-   basic program elements - variables, `if` statements, `while` loops
-   functions, recursion, and parametric polymorphism (i.e. the types and number of arguments determine the used function definition)
-   exceptions, `try-catch` blocks, and unsafe functions - a function, whose name ends with `!` is considered unsafe and can only be used in an unsafe or guarded context
-   lists, operations on them, and `for` loops
-   constants and relative script importing
-   iterators over lists via the methods `__init__()` and `__next__!()` (throwing an exception breaks the `for` loop)
-   classes and associated methods

# Example

This is a code snippet from the file [`dijkstra.ch`](https://github.com/aalyth/Chalcedony/tree/main/examples/dijkstra.ch), which showcases most of the functionalities of the language.

```
class Graph:
    matrix: [[float]]

    (...)

    fn dijkstra!(self, orig: uint, dest: uint) -> float:
        let max_node = self.matrix.len()

        if orig >= max_node || dest >= max_node:
            throw "invalid orig/dest"

        let traversed: [bool] = [false] * max_node
        traversed.set!(true, orig)

        let heap = NodeHeap::from(self.__get_neighbours!(orig))
        while !heap.is_empty():
            let el = heap.next!()
            if el.node == dest:
                return el.dist

            traversed.set!(true, el.node)
            heap.extend(self.__get_neighbours!(el.node, traversed, el.dist))

        # no path is found
        return -1.0

if __name__ == '__main__':
    let graph = Graph::from([ (...) ])
    try:
        print(graph.diijktra!(4, 1))
    catch (exc: exception):
        print("Could not find a path from node 4 to node 1- " + exc)
```
