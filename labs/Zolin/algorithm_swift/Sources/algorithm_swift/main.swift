import Foundation

// Weighted graph
class Graph {
    var adjacencyList: [Int: [(Int, Int)]] = [:]

    func addEdge(_ u: Int, _ v: Int, _ weight: Int) {
        if adjacencyList[u] == nil {
            adjacencyList[u] = []
        }
        adjacencyList[u]?.append((v, weight))
        
        if adjacencyList[v] == nil {
            adjacencyList[v] = []
        }
        adjacencyList[v]?.append((u, weight))
    }
}

// Dijkstra's algorithm
func dijkstra(graph: Graph, startNode: Int) -> [Int: Int] {
    var distances: [Int: Int] = [:]
    var priorityQueue = PriorityQueue<(node: Int, distance: Int)> { $0.distance < $1.distance }

    for node in graph.adjacencyList.keys {
        distances[node] = Int.max
    }
    distances[startNode] = 0
    priorityQueue.enqueue((node: startNode, distance: 0))

    while !priorityQueue.isEmpty {
        let (currentNode, currentDistance) = priorityQueue.dequeue()!
        if currentDistance > distances[currentNode]! {
            continue
        }
        if let neighbors = graph.adjacencyList[currentNode] {
            for neighbor in neighbors {
                let (neighborNode, edgeWeight) = neighbor
                let newDistance = currentDistance + edgeWeight

                if newDistance < distances[neighborNode]! {
                    distances[neighborNode] = newDistance
                    priorityQueue.enqueue((node: neighborNode, distance: newDistance))
                }
            }
        }
    }

    return distances
}

// Priority queue (Min Heap) data structure
struct PriorityQueue<Element> {
    private var elements: [Element] = []
    private let priorityFunction: (Element, Element) -> Bool

    init(priorityFunction: @escaping (Element, Element) -> Bool) {
        self.priorityFunction = priorityFunction
    }

    var isEmpty: Bool {
        return elements.isEmpty
    }

    mutating func enqueue(_ element: Element) {
        elements.append(element)
        swim(elements.count - 1)
    }

    mutating func dequeue() -> Element? {
        guard !elements.isEmpty else {
            return nil
        }
        if elements.count == 1 {
            return elements.removeFirst()
        }
        let first = elements[0]
        elements[0] = elements.removeLast()
        sink(0)
        return first
    }

    private mutating func swim(_ index: Int) {
        var index = index
        while index > 0 {
            let parentIndex = (index - 1) / 2
            if !priorityFunction(elements[index], elements[parentIndex]) {
                return
            }
            elements.swapAt(index, parentIndex)
            index = parentIndex
        }
    }

    private mutating func sink(_ index: Int) {
        var index = index
        while true {
            let leftChildIndex = 2 * index + 1
            let rightChildIndex = 2 * index + 2
            var swapIndex = index

            if leftChildIndex < elements.count && priorityFunction(elements[leftChildIndex], elements[swapIndex]) {
                swapIndex = leftChildIndex
            }

            if rightChildIndex < elements.count && priorityFunction(elements[rightChildIndex], elements[swapIndex]) {
                swapIndex = rightChildIndex
            }

            if swapIndex == index {
                return
            }

            elements.swapAt(index, swapIndex)
            index = swapIndex
        }
    }
}

// Example usage:
let graph = Graph()
graph.addEdge(0, 1, 4)
graph.addEdge(0, 2, 3)
graph.addEdge(1, 2, 1)
graph.addEdge(1, 3, 2)
graph.addEdge(2, 3, 4)
graph.addEdge(3, 4, 2)
graph.addEdge(4, 5, 6)

let startNode = 0
let distances = dijkstra(graph: graph, startNode: startNode)

for (node, distance) in distances {
    if distance == Int.max {
        print("Node \(node) is not reachable from node \(startNode)")
    } else {
        print("Shortest distance from node \(startNode) to node \(node) is \(distance)")
    }
}