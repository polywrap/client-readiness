protocol Feature {
    func runTestCase(input: Any) throws -> Void
}

enum FeatureError: Error {
    case unknownFeature
}

func FeatureFactory(for name: String) throws -> Feature {
    switch name {
    case "uri":
        return Uri()
    default:
        throw FeatureError.unknownFeature
    }
}
