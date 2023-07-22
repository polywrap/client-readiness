package features.config

import kotlinx.serialization.Serializable

@Serializable
data class ResolverExtInput(
    val authority: String,
    val result: String
)

fun resolverExt(input: ResolverExtInput) {
    throw Exception("tryResolveUri is not implemented in the FFI")
}