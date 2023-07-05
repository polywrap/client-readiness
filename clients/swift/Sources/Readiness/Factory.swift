public protocol Feature {
    func runTestCase(input: Any) throws -> Void
}

public enum FeatureError: Error {
    case unknown
    case unsupported
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
    case "config_plugin_package":
        throw FeatureError.unsupported
    case "config_resolver":
        throw FeatureError.unsupported
    case "config_resolver_ext":
        throw FeatureError.unsupported
    case "config_uri_redirect":
        throw FeatureError.unsupported
    case "invoke_plugin":
        return InvokePluginTest()
    case "invoke_wrap_wasm_v0_1":
        return InvokeWrapWasmV0_1Test()
    case "subinvoke_plugin_wrap":
        return SubinvokePluginWrapTest()
    case "subinvoke_wrap_plugin":
        return SubinvokeWrapPluginTest()
    case "subinvoke_wrap_wrap":
        return SubinvokeWrapWrapTest()
    case "uri":
        return UriTest()
    default:
        throw FeatureError.unknown
    }
}
