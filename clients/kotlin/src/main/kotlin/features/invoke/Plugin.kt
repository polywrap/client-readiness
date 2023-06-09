package features.invoke

import io.polywrap.configBuilder.polywrapClient
import io.polywrap.core.resolution.Uri
import kotlinx.serialization.Serializable
import mocks.addPlugin.ArgsAdd
import mocks.addPlugin.addPlugin

@Serializable
data class PluginInput(
    val uri: String,
    val method: String,
    val args: ArgsAdd
)

fun invokePlugin(input: PluginInput) {
    val client = polywrapClient {
        setPackage(input.uri to addPlugin(null))
    }

    println("Invoking ${input.method}")

    val result: Result<Int> = client.invoke(Uri(input.uri), input.method, input.args)

    if (result.isSuccess) {
        println("Received: ${result.getOrThrow()}")
        println("Success!")
    }
}
