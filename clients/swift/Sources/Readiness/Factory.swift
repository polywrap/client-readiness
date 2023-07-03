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
        return ConfigInterfaceImplementations()
    case "uri":
        return UriTest()
    default:
        throw FeatureError.unknownFeature
    }
}
