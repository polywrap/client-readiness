import PolywrapClient
import Foundation

public enum Enum: Int, Codable {
    case option1 = 0
    case option2 = 1
    case option3 = 2

    public init(_ index: Int) {
        switch index {
        case 0:
            self = .option1
        case 1:
            self = .option2
        case 2:
            self = .option3
        default:
            fatalError("Unknown index for enum")
        }
    }
    
    public init(_ value: String) {
        switch value {
        case "OPTION1":
            self = .option1
        case "OPTION2":
            self = .option2
        case "OPTION3":
            self = .option3
        default:
            fatalError("Unknown value for enum")
        }
    }
}

public struct EnumArgs: Codable {
    public var en: Enum

    public init(_ en: Enum) {
        self.en = en
    }
}

struct WrapTypeEnumTest: Feature {
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

        let uri = try Uri("file/\(root)/wraps/enum-type/implementations/as")

        let builder = BuilderConfig().addSystemDefault()
        let client = builder.build()

        let enumInstance: Enum
        if let intValue = argsDict["en"] as? Int {
            enumInstance = Enum(intValue)
        } else if let stringValue = argsDict["en"] as? String {
            enumInstance = Enum(stringValue)
        } else {
            throw InputError.expectedObject // or some other appropriate error
        }

        print("Invoking \(method)")
        let result: Int = try client.invoke(uri: uri, method: method, args: EnumArgs(enumInstance), env: nil)
        print("Result: \(result)")
        print("Success!")
    }
}
