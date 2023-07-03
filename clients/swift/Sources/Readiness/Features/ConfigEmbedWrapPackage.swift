import Foundation
import PolywrapClient

public struct u8MethodArgs: Codable {
    var first: Int
    var second: Int
    
    public init(first: Int, second: Int) {
        self.first = first
        self.second = second
    }
}

struct ConfigEmbedWrapPackageTest: Feature {
   func runTestCase(input: Any) throws -> Void {
        guard let inputObj = input as? [AnyHashable: Any] else {
           throw InputError.expectedObject
        }

        let wrapDir = try Input.expectRootDir(inputObj["directory"]!, rootDir: "./")
        let method = try Input.expectString(inputObj["method"]!)

        guard let argsDict = inputObj["args"] as? [AnyHashable: Any],
            let first = argsDict["first"] as? Int,
            let second = argsDict["second"] as? Int else {
            throw InputError.expectedObject
        }

        print("Reading wrap.info & wrap.wasm from \(wrapDir)")

        let args = u8MethodArgs(first: first, second: second)
        let reader = ResourceReader(bundle: Bundle.module)
        let module = try reader.readFile(wrapDir)

        print("Creating WrapPackage from raw wrap.info & wrap.wasm bytes")

        let package = WasmPackage(reader: nil, module: module)
        print("Adding WrapPackage to client ClientConfig")

        let uri = try! Uri("embed/foo")

        let builder = BuilderConfig()
        builder.addPackage(uri, package)
        let client = builder.build()
        let result: Int? = try? client.invoke(uri: uri, method: method, args: args, env: nil)

        if result != nil {
            print("Success!")
        }
   }
}
