 import PolywrapClient

struct UriTest: Feature {
    func runTestCase(input: Any) throws -> Void {
        guard let inputString = input as? String else {
            return
        }

        let uri = try Uri(inputString)

        print("WRAP URI successfully created.")
        print("uri - \(uri.ffi.toStringUri())")
        print("uri.authority - \(uri.ffi.authority())")
        print("uri.path - \(uri.ffi.path())")
    }
}
