import "iter.ch"

class Edge:
    orig: uint
    dest: uint
    dist: float

    fn new(orig: uint, dest: uint, dist: float) -> Edge:
        return Edge {orig, dest, dist}

class Node:
    node: uint
    dist: float

class NodeHeap:
    heap: [Node]

    fn from(nodes: [Node]) -> NodeHeap:
        return NodeHeap {heap: nodes}

    fn is_empty(self) -> bool:
        return self.heap.len() == 0
            
    fn insert(self, node: Node):
        if self.is_empty():
            self.heap.push_front(node)
            return

        try:
            let front = self.heap.get!(0)
        catch(_: exception):

        if self.heap.len() == 1 && node.dist > front.dist:
            try:
                self.heap.insert!(node, 1) 
            catch(_: exception):
            return

        if node.dist <= front.dist:
            self.heap.push_front(node)
            return

        let left_idx = 0
        let right_idx = self.heap.len() - 1
    
        while left_idx < right_idx:
            let mid_idx: uint = (left_idx + right_idx) / 2
            try:
                let left = self.heap.get!(mid_idx)
                let right = self.heap.get!(mid_idx + 1)
            catch(_: exception):

            if left.dist <= node.dist && node.dist <= right.dist:
                try:
                    self.heap.insert!(node, mid_idx)
                catch(_: exception):
                return

            elif left.dist > node.dist:
                left_idx = mid_idx - 1
            else:
                right_idx = mid_idx

    fn extend(self, nodes: [Node]):
        for node in nodes:
            self.insert(node)

    fn next!(self) -> Node:
        return self.heap.pop_front!()
    
class Graph:
    matrix: [[float]]

    fn from(edges: [Edge]) -> Graph:
        let max_node = 0
        for edge in edges:
            if edge.orig > max_node:
                max_node = edge.orig

            if edge.dest > max_node:
                max_node = edge.dest

        max_node += 1
        # basically creates a 2d matrix with -1 at each field
        let distances: [[float]] = [[-1.0] * max_node] * max_node
                
        for edge in edges:
            try:
                distances.get!(edge.orig).set!(edge.dist, edge.dest)
                distances.get!(edge.dest).set!(edge.dist, edge.orig)
            catch(_: exception):

        return Graph {matrix: distances}

    fn __get_neighbours!(self, node: uint) -> [Node]:
        let neigh = self.matrix.get!(node)
        let result: [Node] = List::new()
        for idx in range(neigh.len()):
            let dist = neigh.get!(idx)
            if dist != -1:
                result.push_back(Node {node: itou(idx), dist})
        return result 

    fn __get_neighbours!(self, node: uint, traversed: [bool], current_dist: float) -> [Node]:
        let neigh = self.matrix.get!(node)
        let result: [Node] = List::new()

        for idx in range(neigh.len()):
            let dist = neigh.get!(idx)
            if traversed.get!(idx) || dist == -1:
                continue

            result.push_back(Node {node: itou(idx), dist: dist + current_dist})

        return result 

    fn dijkstra!(self, orig: uint, dest: uint) -> float:
        let max_node = self.matrix.len()

        if orig >= max_node || dest >= max_node:
            throw "invalid orig/dest"
        
        let traversed: [bool] = List::new()
        for i in range(max_node):
            traversed.push_back(false)
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
    let graph = Graph::from([ 
        Edge::new(0, 1, 3.0), 
        Edge::new(0, 3, 7.0), 
        Edge::new(0, 4, 8.0), 
        Edge::new(1, 3, 4.0), 
        Edge::new(1, 2, 1.0), 
        Edge::new(2, 3, 2.0), 
        Edge::new(3, 4, 3.0)  
    ])

    let dist1 = graph.dijkstra!(0, 2)
    let dist2 = graph.dijkstra!(1, 4)
    print("Dist from 0 to 2: " + dist1)
    print("Dist from 1 to 4: " + dist2)
    assert(dist1 == 4)
    assert(dist2 == 6)

