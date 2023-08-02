import PolywrapClient
import Foundation

public struct ParseArgs: Codable {
    public var value: String

    public init(_ value: String) {
        self.value = value
    }
}

public struct StringifyArgs: Codable {
    public var values: [String]

    public init(_ values: [String]) {
        self.values = values
    }
}

public enum JsonArgs: Codable {
    case parse(ParseArgs)
    case stringify(StringifyArgs)

    public init(from decoder: Decoder) throws {
        let container = try decoder.singleValueContainer()
        if let parseArgs = try? container.decode(ParseArgs.self) {
            self = .parse(parseArgs)
        } else if let stringifyArgs = try? container.decode(StringifyArgs.self) {
            self = .stringify(stringifyArgs)
        } else {
            throw DecodingError.dataCorruptedError(in: container, debugDescription: "Invalid data")
        }
    }

    public func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()
        switch self {
        case .parse(let parseArgs):
            try container.encode(parseArgs)
        case .stringify(let stringifyArgs):
            try container.encode(stringifyArgs)
        }
    }
}

struct WrapTypeJsonTest: Feature {
    func runTestCase(input: Any) throws -> Void {
        guard let inputObj = input as? [AnyHashable: Any] else {
           throw InputError.expectedObject
        }

        let method = try Input.expectString(inputObj["method"]!)

        guard let argsDict = inputObj["args"] as? [AnyHashable: Any] else {
            throw InputError.expectedObject
        }

        guard let resourcePath = Bundle.main.resourcePath else {
            fatalError("Resource folder not found")
        }
        let root = resourcePath + "/Readiness_Readiness.bundle/Contents/Resources"

        let uri = try Uri("file/\(root)/wraps/json-type/implementations/as")

        let builder = BuilderConfig().addSystemDefault()
        let client = builder.build()



        let args: JsonArgs

        if method == "parse" {
            if let parseArgs = argsDict["value"] as? String {
                args = .parse(ParseArgs(parseArgs))
            } else {
                fatalError("Parse args error")
            }
        } else {
            if let stringifyArgs = argsDict["values"] as? [String] {
                args = .stringify(StringifyArgs(stringifyArgs))
            } else {
                fatalError("Stringify args error")
            }
        }

        print("Invoking \(method)")
        let result: String = try client.invoke(uri: uri, method: method, args: args)
        print("Result: \(result)")
        print("Success!")
    }
}
