import PolywrapClient
import Foundation

public struct ObjectProp: Codable {
    public var prop: String
    public var nested: NestedProp

    public init(_ prop: String, _ nested: NestedProp) {
        self.prop = prop
        self.nested = nested
    }
}

public struct NestedProp: Codable {
    public var prop: String
    public init(_ prop: String) {
        self.prop = prop
    }
}

public struct ObjectArgs: Codable {
    public var arg1: ObjectProp
    
    public init(_ arg1: ObjectProp) {
        self.arg1 = arg1
    }
}

public struct ObjectResult: Codable {
    public var prop: String?
    public var nested: NestedProp?
    
    public init(_ prop: String?, _ nested: NestedProp?) {
        self.prop = prop
        self.nested = nested
    }
}

struct WrapTypeObjectTest: Feature {
    func runTestCase(input: Any) throws -> Void {
        guard let inputObj = input as? [AnyHashable: Any] else {
           throw InputError.expectedObject
        }
        let method = try Input.expectString(inputObj["method"]!)

        guard let argsDict = inputObj["args"] as? [AnyHashable: Any],
            let arg1 = argsDict["arg1"] as? [AnyHashable: Any],
            let prop = arg1["prop"] as? String,
            let arg1Nested = arg1["nested"] as? [AnyHashable: Any],
            let nestedProp = arg1Nested["prop"] as? String else {
            throw InputError.expectedObject
        }

        guard let resourcePath = Bundle.main.resourcePath else {
            fatalError("Resource folder not found")
        }
        let root = resourcePath + "/Readiness_Readiness.bundle/Contents/Resources"

        let uri = try Uri("file/\(root)/wraps/object-type/implementations/as")

        let builder = BuilderConfig().addSystemDefault()
        let client = builder.build()

        let nested = NestedProp(nestedProp)
        let arg = ObjectProp(prop, nested)
        let args = ObjectArgs(arg)

        print("Invoking method1")
        let result: [ObjectResult] = try client.invoke(uri: uri, method: method, args: args)

        let encoder = JSONEncoder()
        encoder.outputFormatting = .prettyPrinted // This will make the JSON output indented for readability
        do {
            let jsonData = try encoder.encode(result)
            if let jsonString = String(data: jsonData, encoding: .utf8) {
                let jsonStringWithoutSpaces = jsonString.replacingOccurrences(of: " : ", with: ": ")
                print("Result: \(jsonStringWithoutSpaces)")
            }
        } catch {
            print("Failed to encode result: \(error)")
        }
        print("Success!")
    }
}
