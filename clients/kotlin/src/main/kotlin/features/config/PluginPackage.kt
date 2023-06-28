package features.config

import io.polywrap.configBuilder.polywrapClient
import io.polywrap.core.resolution.Uri
import kotlinx.serialization.Serializable
import mocks.addPlugin.ArgsAdd
import mocks.addPlugin.addPlugin

@Serializable
data class PluginPackageInput(
    val uri: String,
    val method: String,
    val args: ArgsAdd
)

fun pluginPackage(input: PluginPackageInput) {
    println("Creating PluginPackage")

    val pluginPackage = addPlugin(null)

    println("Adding PluginPackage to ClientConfig")

    val client = polywrapClient {
        addPackage(input.uri to pluginPackage)
    }

    println("Invoking PluginPackage")

    val result: Result<Int> = client.invoke(Uri(input.uri), input.method, input.args)

    if (result.isSuccess) {
        println("Success!")
    }
}