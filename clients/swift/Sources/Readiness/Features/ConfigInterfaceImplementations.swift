import PolywrapClient
import Foundation

struct ConfigInterfaceImplementationsTest: Feature {
    func runTestCase(input: Any) throws -> Void {
        guard let inputObj = input as? [AnyHashable: Any] else {
           throw InputError.expectedObject
        }

        let interfaceUri = try Input.expectUri(inputObj["interfaceUri"])
        let implementations: [String] = try Input.expectArray(inputObj["implementations"])

        print("Adding Interface Implementations to ClientConfig")

        let implementationUris = try implementations.compactMap { implementationUri -> Uri in
            guard let uri = try? Uri(implementationUri) else {
                throw UriError.parseError("Error parsing implementation uri \(implementationUri)")
            }
            return uri
        }

        let builder = BuilderConfig()
        let client = builder
            .addInterfaceImplementations(interfaceUri, implementationUris)
            .build()
        
        print("Getting Implementations")

        let result = try client.getImplementations(interfaceUri)

        if !result.isEmpty {
            print("Found \(result.count) Implementations")
            print("Success!")
        }
    }
}
