import PolywrapClient
import Foundation

public struct Arg: Codable {
    public var uint8: UInt8
    public var str: String

    public init(_ uint8: UInt8, _ str: String) {
        self.uint8 = uint8
        self.str = str
    }
}

public struct ImplementationType: Codable {
    public var arg: Arg
    
    public init(_ arg: Arg) {
        self.arg = arg
    }
}

struct WrapFeatureInterfaceInvokeTest: Feature {
    func runTestCase(input: Any) throws -> Void {
        guard let resourcePath = Bundle.main.resourcePath else {
            fatalError("Resource folder not found")
        }
        let root = resourcePath + "/Readiness_Readiness.bundle/Contents/Resources"
        
        let interfaceUri = try Uri("wrap://ens/interface.eth")
        let implementationUri = try Uri("file/\(root)/wraps/interface-invoke/01-implementation/implementations/as")

        let builder = BuilderConfig()
            .addSystemDefault()
            .addInterfaceImplementation(interfaceUri, implementationUri)

        let client = builder.build()
        print("Invoking moduleMethod")

        let arg = Arg(1, "Test String 1")
        let _: Arg = try client.invoke(uri: implementationUri, method: "moduleMethod", args: ImplementationType(arg))
        print("Success!")
    }
}
