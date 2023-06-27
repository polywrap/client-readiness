package features.resolve

import io.polywrap.configBuilder.polywrapClient
import io.polywrap.core.resolution.Uri
import util.uriAuthority

typealias EnsTextRecordInput = String

fun resolveEnsTextRecord(input: EnsTextRecordInput) {
    println("URI Authority: ${uriAuthority(input)}")

    val client = polywrapClient { addDefaults() }

    println("Resolving: $input")

    val result = client.loadWrapper(Uri.fromString(input))

    if (result.isSuccess) {
        println("Received: wrapper")
        println("Success!")
    }
}