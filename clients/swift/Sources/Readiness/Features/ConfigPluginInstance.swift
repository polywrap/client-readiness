import PolywrapClient
import Foundation

struct ConfigPluginInstanceTest: Feature {
    func runTestCase(input: Any) throws -> Void {
        guard let inputObj = input as? [AnyHashable: Any] else {
           throw InputError.expectedObject
        }

        let uri = try Input.expectUri(inputObj["uri"])
        let method = try Input.expectString(inputObj["method"])

        print("Creating Plugin Instance")

        let mockPlugin = MockPlugin(nil)
        mockPlugin.addVoidMethod(name: "increment", closure: mockPlugin.increment)
        
        let pluginWrapper = PluginWrapper(mockPlugin)

        print("Adding Plugin Instance to ClientConfig")

        let builder = BuilderConfig()
        let config = builder
            .addWrapper(uri, pluginWrapper)
        
        let client = config.build()

        for _ in 0..<2 {
            print("Invoking Plugin Instance")
            let _: VoidCodable = try client.invoke(uri: uri, method: method)
            print("counter = \(mockPlugin.counter)")
        }

        print("Success!")
    }
}
