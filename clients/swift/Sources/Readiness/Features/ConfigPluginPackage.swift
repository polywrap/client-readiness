import PolywrapClient
import Foundation

struct ConfigPluginPackageTest: Feature {
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

        print("Creating PluginPackage")

        let mockPlugin = MockPlugin(nil)
        mockPlugin.addVoidMethod(name: "increment", closure: mockPlugin.increment)
        mockPlugin.addMethod(name: "add", closure: mockPlugin.add)

        let pluginPackage = PluginPackage(mockPlugin)

        print("Adding PluginPackage to ClientConfig")

        let builder = BuilderConfig().addPackage(uri, pluginPackage)
        let client = builder.build()

        let args = AddArgs(a: a, b: b)
        print("Invoking PluginPackage")
        let _: Int = try client.invoke(uri: uri, method: method, args: args, env: nil)
        print("Success!")
    }
}
