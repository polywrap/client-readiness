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
        return ConfigPluginPackageTest()
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
    case "wrap_feature_env_vars":
        return WrapFeatureEnvVarsTest()
    case "wrap_feature_interface_invoke":
        return WrapFeatureInterfaceInvokeTest()
    case "wrap_type_bigint":
        return WrapTypeBigintTest()
    case "wrap_type_bignumber":
        return WrapTypeBignumberTest()
    case "wrap_type_bytes":
        return WrapTypeBytesTest()
    case "wrap_type_enum":
        return WrapTypeEnumTest()
    case "wrap_type_ints":
        return WrapTypeIntsTest()
    case "wrap_type_json":
        return WrapTypeJsonTest()
    case "wrap_type_map":
        return WrapTypeMapTest()
    case "wrap_type_object":
        return WrapTypeObjectTest()
    default:
        throw FeatureError.unknown
    }
}
