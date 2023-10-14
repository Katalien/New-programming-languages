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

    // Initialize distances with infinity and the start node with 0
    for node in graph.adjacencyList.keys {
        distances[node] = Int.max
    }
    distances[startNode] = 0

    // Enqueue the start node with a distance of 0
    priorityQueue.enqueue((node: startNode, distance: 0))

    while !priorityQueue.isEmpty {
        let (currentNode, currentDistance) = priorityQueue.dequeue()!

        // Skip nodes whose distance is already smaller
        if currentDistance > distances[currentNode]! {
            continue
        }

        // Iterate through neighboring nodes
        if let neighbors = graph.adjacencyList[currentNode] {
            for neighbor in neighbors {
                let (neighborNode, edgeWeight) = neighbor
                let newDistance = currentDistance + edgeWeight

                // If the new distance is smaller, update the distance and enqueue the neighbor
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


