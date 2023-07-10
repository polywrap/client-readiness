import PolywrapClient
import Foundation

public enum En: Int, Codable {
    case first = 0
    case second = 1

    public init(_ index: Int) {
        switch index {
        case 0:
            self = .first
        case 1:
            self = .second
        default:
            fatalError("unkown index for enum")
        }
    }
    
    public init(_ value: String) {
        switch value {
        case "FIRST":
            self = .first
        case "SECOND":
            self = .second
        default:
            fatalError("unkown value for enum")
        }
    }
}


public struct Object: Codable {
    var prop: String
}

public struct FeatureEnv: Codable {
    public var str: String
    public var optFilledStr: String?
    public var number: Int
    public var bool: Bool
    public var en: En
    public var object: Object
    public var array: [Int]

    public init(
        _ str: String,
        _ optFilledStr: String?,
        _ number: Int,
        _ bool: Bool,
        _ en: En,
        _ object: Object,
        _ array: [Int]
    ) {
        self.str = str
        self.optFilledStr = optFilledStr
        self.number = number
        self.bool = bool
        self.en = en
        self.object = object
        self.array = array
    }
}


struct WrapFeatureEnvVarsTest: Feature {
    func runTestCase(input: Any) throws -> Void {
        guard let inputObj = input as? [AnyHashable: Any] else {
           throw InputError.expectedObject
        }
        
        guard let mainEnvDict = inputObj["mainEnv"] as? [AnyHashable: Any],
            let strMainEnv = mainEnvDict["str"] as? String,
            let optFilledStrMainEnv = mainEnvDict["optFilledStr"] as? String,
            let numberMainEnv = mainEnvDict["number"] as? Int,
            let boolMainEnv = mainEnvDict["bool"] as? Bool,
            let enMainEnv = mainEnvDict["en"] as? String,
            let objectMainEnv = mainEnvDict["object"] as? [AnyHashable: Any],
            let arrayMainEnv = mainEnvDict["array"] as? [Int] else {
            throw InputError.expectedObject
        }
        guard let subinvokeEnvDict = inputObj["subinvokerEnv"] as? [AnyHashable: Any],
            let strSubinvokeEnv = subinvokeEnvDict["str"] as? String,
            let numberSubinvokeEnv = subinvokeEnvDict["number"] as? Int,
            let boolSubinvokeEnv = subinvokeEnvDict["bool"] as? Bool,
            let enSubinvokeEnv = subinvokeEnvDict["en"] as? String,
            let objectSubinvokeEnv = subinvokeEnvDict["object"] as? [AnyHashable: Any],
            let arraySubinvokeEnv = subinvokeEnvDict["array"] as? [Int] else {
            throw InputError.expectedObject
        }
        
        guard let mainProp = objectMainEnv["prop"] as? String else {
            throw InputError.expectedString
        }

        guard let subinvokeProp = objectSubinvokeEnv["prop"] as? String else {
            throw InputError.expectedString
        }

        let objectMain = Object(prop: mainProp)
        let objectSubinvoke = Object(prop: subinvokeProp)

        let mainEnv = FeatureEnv(
            strMainEnv,
            optFilledStrMainEnv,
            numberMainEnv,
            boolMainEnv,
            En(enMainEnv),
            objectMain,
            arrayMainEnv
        )
        
        let subinvokeEnv = FeatureEnv(
            strSubinvokeEnv,
            nil,
            numberSubinvokeEnv,
            boolSubinvokeEnv,
            En(enSubinvokeEnv),
            objectSubinvoke,
            arraySubinvokeEnv
        )
        guard let resourcePath = Bundle.main.resourcePath else {
            fatalError("Resource folder not found")
        }
        let root = resourcePath + "/Readiness_Readiness.bundle/Contents/Resources"

        let mainUri = try Uri("file/\(root)/wraps/env-type/00-main/implementations/as")
        let subinvokeUri = try Uri("file/\(root)/wraps/env-type/02-subinvoker-with-env/implementations/as")

        let builder = try BuilderConfig()
            .addSystemDefault()
            .addEnv(mainUri, mainEnv)
            .addEnv(subinvokeUri, subinvokeEnv)
            .addRedirect(try Uri("mock/main"), mainUri)

        let client = builder.build()
        print("Invoking methodRequireEnv")

        let result: FeatureEnv = try client.invoke(uri: mainUri, method: "methodRequireEnv")
        print("response.str: \(result.str)")
        print("Success!")
        print("Invoking subinvokeMethodRequireEnv")

        let subinvokeResult: FeatureEnv = try client.invoke(uri: subinvokeUri, method: "subinvokeMethodRequireEnv")
        print("response.str: \(subinvokeResult.str)")
        print("Success!")
    }
}
