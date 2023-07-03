 import PolywrapClient

struct UriTest: Feature {
    func runTestCase(input: Any) throws -> Void {
        guard let inputString = input as? String else {
            return
        }
        guard let uri = try? Uri(inputString) else {
            return
        }
        
        print("WRAP URI successfully created.")
        print("uri - \(uri)")
        print("uri.authority - \(uri.ffi.authority())")
        print("uri.path - \(uri.ffi.path())")
    }
}
