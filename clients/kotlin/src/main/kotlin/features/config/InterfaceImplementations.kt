package features.config

import io.polywrap.configBuilder.polywrapClient
import io.polywrap.core.resolution.Uri
import kotlinx.serialization.Serializable

@Serializable
data class InterfaceImplementationsInput(
    val interfaceUri: String,
    val implementations: List<String>
)

fun interfaceImplementations(input: InterfaceImplementationsInput) {
    println("Adding Interface Implementations to ClientConfig")

    val client = polywrapClient {
        addInterfaceImplementations(input.interfaceUri, input.implementations)
    }

    println("Getting Implementations")

    val uri = Uri(input.interfaceUri)
    val result = client.getImplementations(uri).getOrNull()

    if (!result.isNullOrEmpty()) {
        println("Found ${result.size} Implementations")
        println("Success!")
    }
}