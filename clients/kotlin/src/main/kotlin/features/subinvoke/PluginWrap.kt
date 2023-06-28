package features.subinvoke

import io.polywrap.configBuilder.polywrapClient
import io.polywrap.core.InvokeResult
import io.polywrap.core.resolution.Uri
import io.polywrap.wasm.WasmPackage
import kotlinx.serialization.Serializable
import mocks.subinvokePlugin.subinvokePlugin
import util.pathFromTemplate

@Serializable
data class PluginWrapInput(
    val directory: String,
    val method: String,
    val args: ArgsU8Method
) {
    @Serializable
    data class ArgsU8Method(
        val first: Byte,
        val second: Byte
    )
}

fun pluginWrap(input: PluginWrapInput) {
    val wrapDir = pathFromTemplate(input.directory)
    val manifest = wrapDir.resolve("wrap.info").toFile().readBytes()
    val wasmModule = wrapDir.resolve("wrap.wasm").toFile().readBytes()
    val wrapPackage = WasmPackage(manifest, wasmModule)

    val plugin = subinvokePlugin(null)

    val client = polywrapClient {
        addPackage("embed/foo" to wrapPackage)
        addPackage("plugin/bar" to plugin)
    }

    println("Invoking Plugin")

    val result: InvokeResult<Int> = client.invoke(
        uri = Uri("plugin/bar"),
        method = "performSubinvoke",
        args = mapOf(
            "uri" to "embed/foo",
            "method" to input.method,
            "args" to mapOf(
                "first" to input.args.first,
                "second" to input.args.second
            )
        )
    )

    if (result.isSuccess) {
        println("Success!")
    }
}