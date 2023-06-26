package features.config

import kotlinx.serialization.Serializable

@Serializable
data class UriRedirectInput(
    val from: String,
    val to: String
)

fun uriRedirect(input: UriRedirectInput) {
    throw Exception("tryResolveUri is not implemented in the FFI")
}