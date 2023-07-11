import PolywrapClient
import Foundation

public struct IntsArg: Codable {
    public var first: Int
    public var second: Int

    public init(_ first: Int, _ second: Int) {
        self.first = first
        self.second = second
    }
}

struct WrapTypeIntsTest: Feature {
    func runTestCase(input: Any) throws -> Void {
        guard let inputObj = input as? [AnyHashable: Any] else {
           throw InputError.expectedObject
        }

        let method = try Input.expectString(inputObj["method"]!)

        guard let argsDict = inputObj["args"] as? [AnyHashable: Any],
            let first = argsDict["first"] as? Int,
            let second = argsDict["second"] as? Int else {
            throw InputError.expectedObject
        }

        guard let resourcePath = Bundle.main.resourcePath else {
            fatalError("Resource folder not found")
        }
        let root = resourcePath + "/Readiness_Readiness.bundle/Contents/Resources"

        let uri = try Uri("file/\(root)/wraps/numbers-type/implementations/as")

        let builder = BuilderConfig().addSystemDefault()
        let client = builder.build()

        print("Invoking \(method)")
        let result: Int = try client.invoke(uri: uri, method: method, args: IntsArg(first, second), env: nil)
        print("Result: \(result)")
        print("Success!")
    }
}
