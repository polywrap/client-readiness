import PolywrapClient
import Foundation

struct InvokePluginTest: Feature {
    func runTestCase(input: Any) throws -> Void {
        guard let inputObj = input as? [AnyHashable: Any] else {
           throw InputError.expectedObject
        }

        let uri = try Input.expectUri(inputObj["uri"])
        let method = try Input.expectString(inputObj["method"])
        guard let argsDict = inputObj["args"] as? [AnyHashable: Any],
            let a = argsDict["a"] as? Int,
            let b = argsDict["b"] as? Int else {
            throw InputError.expectedObject
        }

        let args = AddArgs(a: a, b: b)
        var plugin = MockPlugin(nil)
        plugin.addMethod(name: "add", closure: plugin.add)

        let package = PluginPackage(plugin)
        let config = BuilderConfig().addPackage(uri, package)
        let client = config.build()

        print("Invoking \(method)")

        let result: Int? = try client.invoke(uri: uri, method: method, args: args)
        guard let r = result else {
            return
        }

        print("Received: \(r)")
        print("Success!")
    }
}
