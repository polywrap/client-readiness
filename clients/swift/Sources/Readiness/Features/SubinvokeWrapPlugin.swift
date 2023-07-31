import PolywrapClient
import Foundation

struct WrapObj {
    public var directory: String
    public var uri: Uri
}

struct SubinvokeWrapPluginTest: Feature {
    func runTestCase(input: Any) throws -> Void {
        guard let inputObj = input as? [AnyHashable: Any] else {
           throw InputError.expectedObject
        }
        let subwrapUri = try Input.expectUri(inputObj["subWrapUri"])
        let method = try Input.expectString(inputObj["method"]!)

       guard let argsDict = inputObj["args"] as? [AnyHashable: Any],
           let a = argsDict["a"] as? Int,
           let b = argsDict["b"] as? Int else {
           throw InputError.expectedObject
       }

        guard let rootWrapObj = inputObj["rootWrap"] as? [AnyHashable: Any],
            let wrapUri = rootWrapObj["uri"] as? String,
            let directory = rootWrapObj["directory"] as? String else {
            throw InputError.expectedObject
        }

        let rootWrapUri = try Input.expectUri(wrapUri)
        let rootWrapDirectory = try Input.expectRootDir(directory, rootDir: "./")

        let reader = ResourceReader(bundle: Bundle.module)
        let module = try reader.readFile(rootWrapDirectory)
        let wasmWrapper = try WasmWrapper(module: module)

        var plugin = MockPlugin(nil)
        plugin.addMethod(name: "add", closure: plugin.add)
        let pluginPackage = PluginPackage(plugin)

        let builder = BuilderConfig()
            .addWrapper(rootWrapUri, wasmWrapper)
            .addPackage(subwrapUri, pluginPackage)
        
        let client = builder.build()

        print("Invoking \(method)")
        let result: Int? = try? client.invoke(uri: rootWrapUri, method: method, args: AddArgs(a: a, b: b))
        if let result = result {
            print("Received: \(result)")
            print("Success!")
        }
    }
}
