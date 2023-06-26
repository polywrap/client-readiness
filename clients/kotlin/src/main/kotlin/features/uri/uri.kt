package features.uri

import io.polywrap.core.resolution.Uri

typealias UriInput = String

fun uri(input: UriInput) {
    val ffiUri: Uri
    try {
        ffiUri = Uri.fromString(input)
    } catch (e: Exception) {
        throw Exception("Invalid URI Received: $input")
    }

    println("WRAP URI successfully created.")

    val uri = ffiUri.toStringUri()
    val authority = uri.substring("wrap://".length).split("/")[0]
    val path = uri.substring("wrap://".length + authority.length + 1)

    println("uri - $uri")
    println("uri.authority - $authority")
    println("uri.path - $path")
}