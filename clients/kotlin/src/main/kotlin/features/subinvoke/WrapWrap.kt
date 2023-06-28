package features.subinvoke

import io.polywrap.configBuilder.polywrapClient
import io.polywrap.core.resolution.Uri
import io.polywrap.wasm.WasmPackage
import kotlinx.serialization.Serializable
import util.pathFromTemplate

@Serializable
data class WrapWrapInput(
    val rootWrap: WrapInput,
    val subWrap: WrapInput,
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

private fun getWrapPackage(dir: String): WasmPackage {
    val wrapDir = pathFromTemplate(dir)
    val manifest = wrapDir.resolve("wrap.info").toFile().readBytes()
    val wasmModule = wrapDir.resolve("wrap.wasm").toFile().readBytes()
    return WasmPackage(manifest, wasmModule)
}

fun wrapWrap(input: WrapWrapInput) {
    val rootWrapPackage = getWrapPackage(input.rootWrap.directory)
    val subWrapPackage = getWrapPackage(input.subWrap.directory)

    val client = polywrapClient {
        addPackage(input.rootWrap.uri to rootWrapPackage)
        addPackage(input.subWrap.uri to subWrapPackage)
    }

    println("Invoking ${input.method}")

    val result: Result<Int> = client.invoke(
        uri = Uri(input.rootWrap.uri),
        method = input.method,
        args = input.args
    )

    if (result.isSuccess) {
        println("Received: ${result.getOrThrow()}")
        println("Success!")
    }
}