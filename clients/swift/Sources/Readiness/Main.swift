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
            print("====================================")
            print("Begin Feature Spec Test Cases [\(name)]")
            print("====================================")

            do {
                let feature = try FeatureFactory(for: name)
                for (testCase, value) in spec.cases {
                    print("$Test Start [\(name).\(testCase)]")
                    do {
                        try feature.runTestCase(input: value.input)
                    } catch {
                        print("!Test Error [\(name).\(testCase)]")
                        print(error)
                    }
                }
            } catch {
                print("Unknown feature: \(name)")
                print(error)
            }

            print("====================================")
            print("End Feature Spec Test Cases [\(name)]")
            print("====================================")
        }
    }
}
