package features.resolve

import io.polywrap.configBuilder.polywrapClient
import io.polywrap.core.resolution.Uri
import util.pathStringFromTemplate
import util.uriAuthority

typealias FileInput = String

fun resolveFile(input: FileInput) {
    val uri = pathStringFromTemplate(input)

    println("URI Authority: ${uriAuthority(uri)}")

    val client = polywrapClient { addDefaults() }

    println("Resolving: $input")

    val result = client.loadWrapper(Uri.fromString(uri))

    if (result.isSuccess) {
        println("Received: wrapper")
        println("Success!")
    }
}