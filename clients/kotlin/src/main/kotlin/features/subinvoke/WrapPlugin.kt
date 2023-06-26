package features.subinvoke

import io.polywrap.configBuilder.polywrapClient
import io.polywrap.core.InvokeResult
import io.polywrap.core.resolution.Uri
import io.polywrap.wasm.WasmPackage
import kotlinx.serialization.Serializable
import mocks.addPlugin.addPlugin
import util.pathFromTemplate

@Serializable
data class WrapPluginInput(
    val rootWrap: WrapInput,
    val subWrapUri: String,
    val method: String,
    val args: AddAndIncrementArgs
) {
    @Serializable
    data class WrapInput(
        val directory: String,
        val uri: String
    )

    @Serializable
    data class AddAndIncrementArgs(
        val a: Int,
        val b: Int
    )
}

fun wrapPlugin(input: WrapPluginInput) {
    val wrapDir = pathFromTemplate(input.rootWrap.directory)
    val manifest = wrapDir.resolve("wrap.info").toFile().readBytes()
    val wasmModule = wrapDir.resolve("wrap.wasm").toFile().readBytes()
    val wrapPackage = WasmPackage(manifest, wasmModule)

    val plugin = addPlugin(null)

    val client = polywrapClient {
        addPackage(input.rootWrap.uri to wrapPackage)
        addPackage(input.subWrapUri to plugin)
    }

    println("Invoking ${input.method}")

    val result: InvokeResult<Int> = client.invoke(
        uri = Uri.fromString(input.rootWrap.uri),
        method = input.method,
        args = input.args
    )

    if (result.isSuccess) {
        println("Received: ${result.getOrThrow()}")
        println("Success!")
    }
}