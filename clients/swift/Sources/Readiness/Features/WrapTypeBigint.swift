import PolywrapClient
import Foundation

public struct Obj: Codable {
    public var prop1: String

    public init(_ prop1: String) {
        self.prop1 = prop1
    }
}

public struct Args: Codable {
    public var arg1: String
    public var obj: Obj

    public init(_ arg1: String, _ obj: Obj) {
        self.arg1 = arg1
        self.obj = obj
    }
}

struct WrapTypeBigintTest: Feature {
    func runTestCase(input: Any) throws -> Void {
        guard let inputObj = input as? [AnyHashable: Any] else {
           throw InputError.expectedObject
        }

        guard let argsDict = inputObj["args"] as? [AnyHashable: Any],
            let arg1 = argsDict["arg1"] as? String,
            let obj = argsDict["obj"] as? [AnyHashable: Any],
            let prop1 = obj["prop1"] as? String else {
            throw InputError.expectedObject
        }

        guard let resourcePath = Bundle.main.resourcePath else {
            fatalError("Resource folder not found")
        }
        let root = resourcePath + "/Readiness_Readiness.bundle/Contents/Resources"

        let uri = try Uri("file/\(root)/wraps/bigint-type/implementations/as")

        let builder = BuilderConfig().addSystemDefault()
        let client = builder.build()

        print("Invoking method")
        let result: String = try client.invoke(uri: uri, method: "method", args: Args(arg1, Obj(prop1)), env: nil)
        print("Result: \(result)")
        print("Success!")
    }
}
