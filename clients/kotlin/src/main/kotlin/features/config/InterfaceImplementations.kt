package features.config

import io.polywrap.configBuilder.polywrapClient
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

    val result = client.getImplementations(input.interfaceUri).getOrNull()

    if (!result.isNullOrEmpty()) {
        println("Found ${result.size} Implementations")
        println("Success!")
    }
}