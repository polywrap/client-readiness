import PolywrapClient
import Foundation


public class SubinvokePlugin: PluginModule {
    public var uri: Uri
    public var method: String
    public var args: [UInt8]

    public init(_ uri: Uri, _ method: String, _ args: [UInt8]) {
        self.uri = uri
        self.method = method
        self.args = args
    }

    public func performSubinvoke(args: EmptyArgs, _: EmptyEnv?, invoker: Invoker) throws -> Bool {
        let result = try invoker.invoke(uri: self.uri, method: self.method, args: self.args, env: nil, resolution_context: nil)
        return true
    }
}

struct SubinvokePluginWrapTest: Feature {
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
        let encoded_args = try encode(value: args)
        
        let pluginUri = try Uri("plugin/bar")
        let wrapperUri = try Uri("embed/foo")

        let subinvokePlugin = SubinvokePlugin(wrapperUri, method, encoded_args)
        subinvokePlugin.addMethod(name: "performSubinvoke", closure: subinvokePlugin.performSubinvoke)

        let pluginPackage = PluginPackage(subinvokePlugin)

        let reader = ResourceReader(bundle: Bundle.module)
        let module = try reader.readFile(wrapDir)
        let wasmPackage = WasmPackage(reader: nil, module: module)
        
        let builder = BuilderConfig().addPackage(wrapperUri, wasmPackage).addPackage(pluginUri, pluginPackage)

        let client = builder.build()
        
        print("Invoking plugin")
        let result: Bool? = try? client.invoke(uri: pluginUri, method: "performSubinvoke")

        guard let r = result else {
            return
        }
        
        print("Received \(r)")
        print("Success")
    }
}
