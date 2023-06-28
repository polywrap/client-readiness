package features.config

import kotlinx.serialization.Serializable

@Serializable
data class ResolverInput(
    val authority: String,
    val result: String
)

fun resolver(input: ResolverInput) {
    throw Exception("tryResolveUri is not implemented in the FFI")
//    println("Adding Resolver to ClientConfig")
//
//    val resolver: UriResolver = object : UriResolver {
//        override fun tryResolveUri(
//            uri: FfiUri,
//            invoker: FfiInvoker,
//            resolutionContext: FfiUriResolutionContext
//        ): FfiUriPackageOrWrapper {
//            val isAuthority = uri.toStringUri().startsWith("wrap://${input.authority}")
//            val response = if (isAuthority) {
//                Uri(input.result)
//            } else {
//                uri
//            }
//
//            return UriPackageOrWrapper.UriValue(response)
//        }
//    }
//
//    val client = polywrapClient {
//        addResolver(resolver)
//    }
//
//    println("Resolving a wrap://${input.authority} URI")
//
//    val res = client.loadWrapper(
//        uri = Uri("wrap://${input.authority}/foo")
//    )

    //  console.log(res)
    //
    //  if (res.ok && res.value.type === "uri") {
    //    console.log(`Received URI '${res.value.uri}'`);
    //    console.log("Success!");
    //  }
}
