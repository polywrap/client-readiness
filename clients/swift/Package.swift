// swift-tools-version: 5.7
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "Readiness",
    platforms: [.iOS(.v15), .macOS(.v10_15)],
    dependencies: [
        // Dependencies declare other packages that this package depends on.
        .package(url: "https://github.com/jpsim/Yams.git", from: "5.0.6"),
        .package(url: "https://github.com/polywrap/swift-client", branch: "main")
    ],
    targets: [
        // Targets are the basic building blocks of a package. A target can define a module or a test suite.
        // Targets can depend on other targets in this package, and on products in packages this package depends on.
        .executableTarget(
            name: "Readiness",
            dependencies: [
                .product(name: "PolywrapClient", package: "swift-client"),
                "Yams"
            ],
            resources: [ .copy("Specs"), .copy("wraps") ]
        )
    ]
)
