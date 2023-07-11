import PolywrapClient
import Foundation

struct ConfigUriRedirectTest: Feature {
    func runTestCase(input: Any) throws -> Void {
        guard let inputObj = input as? [AnyHashable: Any] else {
           throw InputError.expectedObject
        }

        let from = try Input.expectUri(inputObj["from"])
        let to = try Input.expectUri(inputObj["to"])

        print("Adding URI Redirect to ClientConfig")

        let config = BuilderConfig().addRedirect(from, to)
        let client = config.build()

        print("Resolving Redirect")
        // let result = client.tryResolveUri()
    }
}
