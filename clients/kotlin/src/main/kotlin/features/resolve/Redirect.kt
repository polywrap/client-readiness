package features.resolve

import kotlinx.serialization.Serializable

@Serializable
data class RedirectInput(
    val from: String,
    val to: String
)

fun resolveRedirect(input: RedirectInput) {
    throw Exception("tryResolveUri is not implemented in the FFI")
}