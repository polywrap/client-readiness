import PolywrapClient
import Foundation

public struct BytesProp: Codable {
    public var prop: [Int]

    public init(_ prop: [Int]) {
        self.prop = prop
    }
}

public struct BytesArgs: Codable {
    public var arg: BytesProp

    public init(_ arg: BytesProp) {
        self.arg = arg
    }
}

struct WrapTypeBytesTest: Feature {
    func runTestCase(input: Any) throws -> Void {
        guard let inputObj = input as? [AnyHashable: Any] else {
           throw InputError.expectedObject
        }

        guard let argsDict = inputObj["args"] as? [AnyHashable: Any],
            let arg = argsDict["arg"] as? [AnyHashable: Any],
            let prop = arg["prop"] as? [Int] else {
            throw InputError.expectedObject
        }

        guard let resourcePath = Bundle.main.resourcePath else {
            fatalError("Resource folder not found")
        }
        let root = resourcePath + "/Readiness_Readiness.bundle/Contents/Resources"

        let uri = try Uri("file/\(root)/wraps/bytes-type/implementations/as")

        let builder = BuilderConfig().addSystemDefault()
        let client = builder.build()

        print("Invoking bytesMethod")
        let result: Data = try client.invoke(uri: uri, method: "bytesMethod", args: BytesArgs(BytesProp(prop)), env: nil)
        let bytesAsString = [UInt8](result).map { String($0) }.joined(separator: ",")

        print("Result: [\(bytesAsString)]")
        print("Success!")
    }
}
