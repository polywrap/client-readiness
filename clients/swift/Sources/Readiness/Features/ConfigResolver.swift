import PolywrapClient
import Foundation

//class MockUriPackageOrWrapper: FfiUriPackageOrWrapper {
//    public let uri: Uri
//    public init(_ uri: String) throws {
//        self.uri = try Uri(uri)
//    }
//
//    func getKind() -> FfiUriPackageOrWrapperKind {
//        return FfiUriPackageOrWrapperKind.uri
//    }
//
//    func asUri() -> FfiUri {
//        return self.uri.ffi
//    }
//
//    func asWrapper() -> FfiUriWrapper {
//        fatalError("Not a wrapper")
//    }
//
//    func asPackage() -> FfiUriWrapPackage {
//        fatalError("Not a package")
//    }
//}


struct ConfigResolverTest: Feature {
    func runTestCase(input: Any) throws -> Void {
        guard let inputObj = input as? [AnyHashable: Any] else {
           throw InputError.expectedObject
        }

        let authority = try Input.expectString(inputObj["authority"])
        let result = try Input.expectString(inputObj["result"])

        print("Adding Resolver to ClientConfig")

//        class Resolver: FfiUriResolver {
//            var authority: String
//            var result: String
//
//            public init(_ authority: String, _ result: String) {
//                self.authority = authority
//                self.result = result
//            }
//
//            func tryResolveUri(
//                uri: FfiUri,
//                invoker: FfiInvoker,
//                resolutionContext: FfiUriResolutionContext
//            ) throws -> FfiUriPackageOrWrapper {
//                if uri.authority() == self.authority {
//
//                }
//            }
//        }
//
//        let resolver: FfiUriResolver = Resolver(authority, result)
//        let config = BuilderConfig().addResolver(resolver)
//        let client = config.build()
        
//        print("Resolving a wrap://\(authority) URI")
//        let result = client.tryResolveUri()
        
    }
}
