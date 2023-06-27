package features.uri

import io.polywrap.core.resolution.Uri
import util.uriAuthority
import util.uriPath

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
    val authority = uriAuthority(uri)
    val path = uriPath(uri)

    println("uri - $uri")
    println("uri.authority - $authority")
    println("uri.path - $path")
}