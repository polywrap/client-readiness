import Foundation
import PolywrapClient

struct InvokeWrapWasmV0_1Test: Feature {
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

        let args = u8MethodArgs(first: first, second: second)
        let reader = ResourceReader(bundle: Bundle.module)
        let module = try reader.readFile(wrapDir)

        let package = WasmPackage(reader: nil, module: module)
        let uri = try! Uri("embed/foo")

        let builder = BuilderConfig().addPackage(uri, package)

        let client = builder.build()

        print("Invoking \(method)")
        let result: Int? = try? client.invoke(uri: uri, method: method, args: args)
        guard let r = result else {
            return
        }

        print("Received: \(r)")
        print("Success!")
   }
}
