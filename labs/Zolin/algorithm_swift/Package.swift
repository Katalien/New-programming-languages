// swift-tools-version:5.5
import PackageDescription

let package = Package(
    name: "algorithm_swift",
    products: [
        .executable(name: "algorithm_swift", targets: ["algorithm_swift"]),
    ],
    targets: [
        .target(
            name: "algorithm_swift",
            dependencies: [
                // Add your target's dependencies here
            ]
        ),
    ]
)
