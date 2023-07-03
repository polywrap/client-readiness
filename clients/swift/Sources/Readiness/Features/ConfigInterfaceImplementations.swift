import PolywrapClient
import Foundation

public struct Env: Codable {
    var str: String
    var num: Int
    
    public init(str: String, num: Int) {
        self.str = str
        self.num = num
    }
}

struct ConfigInterfaceImplementations: Feature {
    func runTestCase(input: Any) throws -> Void {
        guard let inputObj = input as? [AnyHashable: Any] else {
           throw InputError.expectedObject
        }

        let interfaceUri = try Input.expectUri(inputObj["interfaceUri"])
        let implementations: [String] = try Input.expectArray(inputObj["implementations"])

        print("Adding Interface Implementations to ClientConfig")

         let builder = BuilderConfig()
    }
}
