import PolywrapClient
import Foundation

public struct MapArgs: Codable {
    public var map: Dictionary<String, Int>
    
    public init(_ map: Dictionary<String, Int>) {
        self.map = map
    }
}

struct WrapTypeMapTest: Feature {
    func runTestCase(input: Any) throws -> Void {
        guard let inputObj = input as? [AnyHashable: Any] else {
           throw InputError.expectedObject
        }

        guard let mapDict = inputObj["map"] as? [AnyHashable: Any] else {
            throw InputError.expectedObject
        }

        guard let resourcePath = Bundle.main.resourcePath else {
            fatalError("Resource folder not found")
        }
        let root = resourcePath + "/Readiness_Readiness.bundle/Contents/Resources"

        let uri = try Uri("file/\(root)/wraps/map-type/implementations/as")

        let builder = BuilderConfig().addSystemDefault()
        let client = builder.build()

        var map: Dictionary<String, Int> = [:]

        for (key, value) in mapDict {
            if let keyString = key as? String, let valueInt = value as? Int {
                map[keyString] = valueInt
            } else {
                fatalError("wrong map filling")
            }
        }
        print("Invoking returnMap")
        let result: Dictionary<String, Int> = try client.invoke(uri: uri, method: "returnMap", args: MapArgs(map), env: nil)
        let sortedKeys = result.keys.sorted()
        for key in sortedKeys {
            if let value = result[key] {
                print("key '\(key)' = \(value)")
            }
        }
        print("Success!")
    }
}
