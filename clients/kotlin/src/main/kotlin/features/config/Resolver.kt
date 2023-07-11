package features.config

import io.polywrap.configBuilder.polywrapClient
import io.polywrap.core.resolution.UriPackageOrWrapper
import io.polywrap.core.resolution.UriResolver
import kotlinx.serialization.Serializable
import uniffi.polywrap_native.FfiInvoker
import uniffi.polywrap_native.FfiUri
import uniffi.polywrap_native.FfiUriPackageOrWrapper
import uniffi.polywrap_native.FfiUriResolutionContext

@Serializable
data class ResolverInput(
    val authority: String,
    val result: String
)

fun resolver(input: ResolverInput) {
    println("Adding Resolver to ClientConfig")

    val resolver: UriResolver = object : UriResolver {
        override fun tryResolveUri(
            uri: FfiUri,
            invoker: FfiInvoker,
            resolutionContext: FfiUriResolutionContext
        ): FfiUriPackageOrWrapper {
            val isAuthority = uri.toStringUri().startsWith("wrap://${input.authority}")
            val response = if (isAuthority) {
                FfiUri.fromString(input.result)
            } else {
                uri
            }
            return UriPackageOrWrapper.UriValue(response)
        }

        override fun close() {}
    }

    polywrapClient {
        addResolver(resolver)
    }

    println("Success!")
}
