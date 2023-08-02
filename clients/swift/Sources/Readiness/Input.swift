import Foundation
import PolywrapClient


enum InputError: Error {
        case expectedObject
        case expectedRootDir
        case expectedString
        case expectedUri
        case expectedArray
}

class Input {
    static func expectRootDir(_ input: Any?, rootDir: String) throws -> String {
        if let input = input as? String, input.hasPrefix("$ROOT/") {
            return input.replacingOccurrences(of: "$ROOT/", with: rootDir)
        } else {
            throw InputError.expectedRootDir
        }
    }

    static func expectString(_ input: Any?) throws -> String {
        if let input = input as? String {
            return input
        } else {
            throw InputError.expectedString
        }
    }

    static func expectUri(_ input: Any?) throws -> Uri {
        guard let inputString = input as? String else {
            throw InputError.expectedUri
        }
        do {
            return try Uri(inputString)
        } catch {
            throw InputError.expectedUri
        }
    }

    static func expectArray<T>(_ input: Any?) throws -> [T] {
        if let input = input as? [T] {
            return input
        } else {
            throw InputError.expectedArray
        }
    }
}
