import Foundation
import PolywrapClient

public struct AddArgs: Codable {
    var a: Int
    var b: Int
    
    public init(a: Int, b: Int) {
        self.a = a
        self.b = b
    }
}

public struct EmptyArgs: Codable {}
public struct EmptyEnv: Codable {}

public class MockPlugin: PluginModule {
    public var counter = 0
    public init(_ value: Int?) {
        if value != nil {
            self.counter = value!
        }
    }

    public func add(_ args: AddArgs, _ env: EmptyEnv?, _ invoker: Invoker) -> Int {
        return args.a + args.b
    }

    public func increment(_ args: EmptyArgs, _ env: EmptyEnv?, _ invoker: Invoker) throws {
        self.counter += 1
    }
}