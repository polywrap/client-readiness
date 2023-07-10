import Foundation

@main
public struct Readiness {
    public static func main() async throws {
        let filter: String? = CommandLine.arguments.count > 1 ? CommandLine.arguments[1] : nil

        guard let specs = try? loadFeatureSpecs("./Specs") else {
            print("Failed to load feature specs")
            return
        }

        for (name, spec) in specs {
            if let filter = filter, filter != name {
                continue
            }

            do {
                let feature = try FeatureFactory(for: name)
                print("====================================")
                print("Begin Feature Spec Test Cases [\(name)]")
                print("====================================")

                for (testCase, value) in spec.cases {
                    print("$Test Start [\(name).\(testCase)]")
                    do {
                        try feature.runTestCase(input: value.input)
                    } catch {
                        printError("!Test Error [\(name).\(testCase)]")
                        printError(error)
                    }
                }
                print("====================================")
                print("End Feature Spec Test Cases [\(name)]")
                print("====================================")
            } catch FeatureError.unsupported {
                print("Unsupported feature: \(name)")
            } catch {
                print("Unknown feature: \(name)")
                print(error)
            }
        }
    }
}

public func printError(_ items: Any...) {
    let output = items.map { "\($0)" }.joined(separator: " ")
    if let data = (output + "\n").data(using: .utf8) {
        FileHandle.standardError.write(data)
    }
}
