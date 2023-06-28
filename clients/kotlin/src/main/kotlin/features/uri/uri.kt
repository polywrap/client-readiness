package features.uri

import io.polywrap.core.resolution.Uri

typealias UriInput = String

fun uri(input: UriInput) {
    val uri = Uri(input)

    println("WRAP URI successfully created.")

    println("uri - $uri")
    println("uri.authority - ${uri.authority}")
    println("uri.path - ${uri.path}")
}