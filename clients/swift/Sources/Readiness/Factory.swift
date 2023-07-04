public protocol Feature {
    func runTestCase(input: Any) throws -> Void
}

public enum FeatureError: Error {
    case unknownFeature
}

public func FeatureFactory(for name: String) throws -> Feature {
    switch name {
    case "config_embed_wrap_package":
        return ConfigEmbedWrapPackageTest()
    case "config_env_variables":
        return ConfigEnvVariablesTest()
    case "config_interface_implementations":
        return ConfigInterfaceImplementationsTest()
    case "config_plugin_instance":
        return ConfigPluginInstanceTest()
//    case "config_plugin_package":
//        return ConfigPluginPackageTest()
//    case "config_resolver":
//        return ConfigResolverTest()
//    case "config_resolver_ext":
//        return ConfigResolverExtTest()
//    case "config_uri_redirect":
//        return ConfigUriRedirectTest()
    case "invoke_plugin":
        return InvokePluginTest()
    case "invoke_wrap_wasm_v0_1":
        return InvokeWrapWasmV0_1Test()
    case "uri":
        return UriTest()
    default:
        throw FeatureError.unknownFeature
    }
}
