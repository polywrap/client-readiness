import PolywrapClient
import Foundation

public struct CustomEnv: Codable {
    var str: String
    var num: Int
    
    public init(str: String, num: Int) {
        self.str = str
        self.num = num
    }
}

struct ConfigEnvVariablesTest: Feature {
    func runTestCase(input: Any) throws -> Void {
        guard let inputObj = input as? [AnyHashable: Any] else {
           throw InputError.expectedObject
        }

        let uri = try Input.expectUri(inputObj["uri"])

        guard let envDict = inputObj["env"] as? [AnyHashable: Any],
             let str = envDict["str"] as? String,
             let num = envDict["num"] as? Int else {
           throw InputError.expectedObject
        }

        let env = Env(str: str, num: num)

        print("Adding Env to ClientConfig")

        let builder = BuilderConfig()
        try builder.addEnv(uri, env)
        let client = builder.build()

        print("Fetching Env")

        guard let result: CustomEnv = client.getEnvByUri(uri) else {
            let error = NSError(domain: "", code: 1, userInfo: [NSLocalizedDescriptionKey : "No environment variables found for \(uri)"])
            throw error
        }

        print("env.str = \(result.str)")
        print("env.num = \(result.num)")
        print("Success!")
    }
}
