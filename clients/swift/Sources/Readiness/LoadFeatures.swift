import Foundation
import Yams

struct TestCase {
    let input: Any
}

struct Spec {
    let required: Bool
    let cases: [String: TestCase]
}

typealias FeatureSpecs = [String: Spec]

func loadFeatureSpecs(_ path: String) throws -> FeatureSpecs {
    var featureSpecs: FeatureSpecs = [:]

    let fileManager = FileManager.default
    let directoryURL = URL(fileURLWithPath: path)

    guard let specFiles = try? fileManager.contentsOfDirectory(at: directoryURL, includingPropertiesForKeys: nil) else {
        throw NSError(domain: "", code: -1, userInfo: [NSLocalizedDescriptionKey: "Unable to read directory contents."])
    }

    for specFileURL in specFiles {
        let specYaml = try String(contentsOf: specFileURL, encoding: .utf8)

        guard var spec = try Yams.load(yaml: specYaml) as? [String: Any] else {
            throw NSError(domain: "", code: -1, userInfo: [NSLocalizedDescriptionKey: "Failed to load feature-spec \(specFileURL.lastPathComponent), must be an object."])
        }

        guard let required = spec["required"] as? Bool else {
            throw NSError(domain: "", code: -1, userInfo: [NSLocalizedDescriptionKey: "Failed to load feature-spec \(specFileURL.lastPathComponent), must have property 'required'."])
        }

        guard let casesData = spec["cases"] as? [String: [String: Any]] else {
            throw NSError(domain: "", code: -1, userInfo: [NSLocalizedDescriptionKey: "Failed to load feature-spec \(specFileURL.lastPathComponent), must have property 'cases'."])
        }

        var cases: [String: TestCase] = [:]
        for (key, value) in casesData {
            if let input = value["input"] {
                cases[key] = TestCase(input: input)
            } else {
                throw NSError(domain: "", code: -1, userInfo: [NSLocalizedDescriptionKey: "Failed to load feature spec test case \(specFileURL.lastPathComponent).cases.\(key), missing 'input' property"])
            }
        }

        let specName = specFileURL.deletingPathExtension().lastPathComponent
        featureSpecs[specName] = Spec(required: required, cases: cases)
    }

    return featureSpecs
}