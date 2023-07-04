import PolywrapClient
import Foundation

struct Args: Codable {}
struct PluginEnv: Codable {}

class MockPlugin: PluginModule {
    var counter = 0
    
    public override init() {
        super.init()
    }

    public func increment(_ args: Args?, _ env: PluginEnv?, _ invoker: Invoker) throws {
        self.counter += 1
    }
}

struct ConfigPluginInstanceTest: Feature {
    func runTestCase(input: Any) throws -> Void {
        guard let inputObj = input as? [AnyHashable: Any] else {
           throw InputError.expectedObject
        }

        let uri = try Input.expectUri(inputObj["uri"])
        let method = try Input.expectString(inputObj["method"])

        print("Creating Plugin Instance")

        let mockPlugin = MockPlugin()
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
