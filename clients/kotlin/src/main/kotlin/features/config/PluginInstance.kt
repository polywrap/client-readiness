package features.config

import io.polywrap.configBuilder.polywrapClient
import io.polywrap.core.resolution.Uri
import io.polywrap.plugin.PluginPackage
import kotlinx.serialization.Serializable
import mocks.incrementPlugin.IncrementPlugin
import mocks.mockManifest

@Serializable
data class PluginInstanceInput(
    val uri: String,
    val method: String
)

fun pluginInstance(input: PluginInstanceInput) {
    println("Creating Plugin Instance")

    val plugin = IncrementPlugin()

    println("Adding Plugin Instance to ClientConfig")

    val pluginPackage = PluginPackage(
        pluginModule = plugin,
        manifest = mockManifest
    )

    val client = polywrapClient {
        addPackage(input.uri to pluginPackage)
    }

    for (i in 0 until 2) {
        println("Invoking Plugin Instance")
        val result: Result<Unit> = client.invoke(Uri.fromString(input.uri), input.method, null, null)
        if (result.isSuccess) {
            println("counter = ${plugin.counter}")
        }
    }

    println("Success!")
}