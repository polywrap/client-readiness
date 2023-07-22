import PolywrapClient
import Foundation

struct TryResolveUriArgs: Encodable {
    var authority: String
    
    public init(authority: String) {
        self.authority = authority
    }
}

struct TryResolveUriResponse: Codable {
    var manifest: String?
    var uri: String?

    public init(_ uri: String?) {
        self.uri = uri
    }
}

struct ConfigResolverExtTest: Feature {
    func runTestCase(input: Any) throws -> Void {
        guard let inputObj = input as? [AnyHashable: Any] else {
           throw InputError.expectedObject
        }

        let authority = try Input.expectString(inputObj["authority"])
        let result = try Input.expectString(inputObj["result"])

        print("Creating CustomResolverExt Plugin")
    
        class ResolverPlugin: PluginModule {
            public var authority: String
            public var result: String

            public init(_ authority: String, _ result: String) {
                self.authority = authority
                self.result = result
            }

            public func tryResolveUri(_ args: TryResolveUriArgs?, _ env: EmptyEnv?, _ invoker: Invoker) throws -> TryResolveUriResponse {
                if args!.authority == self.authority {
                    return TryResolveUriResponse(self.result)
                }
                return TryResolveUriResponse(nil)
            }
        }

        let plugin = ResolverPlugin(authority, result)
        let package = PluginPackage(plugin)
        let pluginUri = try Uri("plugin/custom-resolver")
        let interfaceUri = try Uri("ens/uri-resolver.core.polywrap.eth")

        print("Adding CustomResolverExt & ExtendableUriResolver to ClientConfig")

        let config = BuilderConfig()
            .addPackage(pluginUri, package)
            .addInterfaceImplementation(interfaceUri, pluginUri)
        let client = config.build()
        
        print("Resolving a wrap://\(authority) URI")
//        let result = client.tryResolveUri()
        
    }
}
