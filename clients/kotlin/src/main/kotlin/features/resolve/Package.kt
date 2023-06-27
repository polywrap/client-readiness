package features.resolve

import kotlinx.serialization.Serializable

@Serializable
data class PackageInput(
    val directory: String,
    val uri: String
)

fun resolvePackage(input: PackageInput) {
    throw Exception("tryResolveUri is not implemented in the FFI")
}