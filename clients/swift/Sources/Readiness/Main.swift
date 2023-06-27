import Foundation


@main
public struct Readiness {
    public static func main() async throws {
        let filter: String? = CommandLine.arguments.count > 1 ? CommandLine.arguments[1] : nil
        print(filter ?? "Filter is undefined")

        let specs = try? loadFeatureSpecs("../../specs/")
        for (specName, value) in specs {
            if let filter = filter, filter != specName {
                continue
            }

        guard let specValue = value as? Spec,
              let spec = TestSpec(
                required: specValue["required"] as? Bool ?? false,
                cases: specValue["cases"] as? [String: TestCase] ?? [:]
            ) else {
                continue
            }
        }

        print(specValue)
    }
}

// struct Feature {
//     var runTestCase: ((String) async throws -> Void)?

//     init(path: String) async throws {
//         // Implementation of importing feature from file goes here
//     }
// }

// struct TestSpec {
//     let required: Bool
//     let cases: [String: Any]
// }

// func main() async {
//     let filter = CommandLine.arguments.count > 2 ? CommandLine.arguments[2] : nil

//     let specs = loadFeatureSpecs("../../../specs")

//     for (specName, value) in specs {
//         if let filter = filter, filter != specName {
//             continue
//         }

//         guard let specValue = value as? [String: Any],
//               let spec = TestSpec(required: specValue["required"] as? Bool ?? false,
//                                   cases: specValue["cases"] as? [String: Any] ?? [:]) else {
//             continue
//         }

//         do {
//             let feature = try await Feature(path: "./features/\(specName)")

//             guard let runTestCase = feature.runTestCase else {
//                 if !spec.required {
//                     continue
//                 }
//                 throw NSError(domain: "", code: -1, userInfo: [NSLocalizedDescriptionKey: "Invalid feature definition: \(feature)"])
//             }

//             print("====================================")
//             print("Begin Feature Spec Test Cases [\(specName)]")
//             print("====================================")

//             for (testCase, caseValue) in spec.cases {
//                 guard let input = caseValue["input"] as? String else {
//                     continue
//                 }
//                 print("$Test Start [\(specName).\(testCase)]")

//                 do {
//                     try await runTestCase(input)
//                 } catch {
//                     print("!Test Error [\(specName).\(testCase)]")
//                     print(error)
//                 }
//             }

//             print("====================================")
//             print("End Feature Spec Test Cases [\(specName)]")
//             print("====================================")
//         } catch {
//             print(error)
//         }
//     }
// }