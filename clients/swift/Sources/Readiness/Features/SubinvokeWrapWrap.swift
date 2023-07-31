import PolywrapClient
import Foundation
 
struct SubinvokeWrapWrapTest: Feature {
    func runTestCase(input: Any) throws -> Void {
        guard let inputObj = input as? [AnyHashable: Any] else {
           throw InputError.expectedObject
        }

        let method = try Input.expectString(inputObj["method"]!)

        guard let argsDict = inputObj["args"] as? [AnyHashable: Any],
            let a = argsDict["a"] as? Int,
            let b = argsDict["b"] as? Int else {
            throw InputError.expectedObject
        }
        
        let rootWrap = try verifyWrap(inputObj["rootWrap"]!)
        let subWrap = try verifyWrap(inputObj["subWrap"]!)

        let rootWrapUri = rootWrap.uri
        let rootWrapPackage = try loadWasmPackage(rootWrap.directory)

        let subWrapUri = subWrap.uri
        let subWrapPackage = try loadWasmPackage(subWrap.directory)

        let builder = BuilderConfig()
            .addPackage(rootWrapUri, rootWrapPackage)
            .addPackage(subWrapUri, subWrapPackage)
        
        let client = builder.build()

        print("Invoking \(method)")
        let result: Int? = try? client.invoke(uri: rootWrapUri, method: method, args: AddArgs(a: a, b: b))
        if let result = result {
            print("Received: \(result)")
            print("Success!")
        }
    }

    func verifyWrap(_ wrap: Any) throws -> WrapObj {
        guard let wrapObj = wrap as? [AnyHashable: Any] else {
            throw InputError.expectedObject
        }

        let directory = try Input.expectRootDir(wrapObj["directory"]!, rootDir: "./")
        let uri = try Input.expectUri(wrapObj["uri"]!)
        return WrapObj(directory: directory, uri: uri)
    }

    func loadWasmPackage(_ wrapDir: String) throws -> WasmPackage {
        let reader = ResourceReader(bundle: Bundle.module)
        let module = try reader.readFile(wrapDir)
        return WasmPackage(reader: nil, module: module)
    }
}
