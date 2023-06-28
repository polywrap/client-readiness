package features.config

import io.polywrap.configBuilder.polywrapClient
import io.polywrap.core.resolution.Uri
import kotlinx.serialization.Contextual
import kotlinx.serialization.Serializable

@Serializable
data class EnvVariablesInput(
    val uri: String,
    val env: Map<String, @Contextual Any>
)

fun envVariables(input: EnvVariablesInput) {
    println("Adding Env to ClientConfig")

    val client = polywrapClient {
        addEnv(input.uri to input.env)
    }

    println("Fetching Env")

    val uri = Uri(input.uri)
    val result = client.getEnvByUri(uri).getOrNull()

    if (result != null) {
        for (key in result.keys) {
            println("env.$key = ${result[key]}")
        }
        println("Success!")
    }
}
