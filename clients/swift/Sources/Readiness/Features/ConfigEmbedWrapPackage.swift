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
        guard let inputObj = input as? [AnyHashable: Any],
            let directory = inputObj["directory"] as? String else {
           throw InputError.expectedObject
        }


        print("Reading wrap.info & wrap.wasm from \(directory)")

        let wrapDir = try Input.expectRootDir(directory, rootDir: "./")
        let method = try Input.expectString(inputObj["method"]!)

        guard let argsDict = inputObj["args"] as? [AnyHashable: Any],
            let first = argsDict["first"] as? Int,
            let second = argsDict["second"] as? Int else {
            throw InputError.expectedObject
        }

        let args = u8MethodArgs(first: first, second: second)
        let reader = ResourceReader(bundle: Bundle.module)
        let module = try reader.readFile(wrapDir)

        print("Creating WrapPackage from raw wrap.info & wrap.wasm bytes")

        let package = WasmPackage(reader: nil, module: module)
        print("Adding WrapPackage to ClientConfig")

        let uri = try! Uri("embed/foo")

        let builder = BuilderConfig().addPackage(uri, package)

        let client = builder.build()
        print("Invoking WrapPackage")
        let result: Int? = try? client.invoke(uri: uri, method: method, args: args)

        if result != nil {
            print("Success!")
        }
   }
}
