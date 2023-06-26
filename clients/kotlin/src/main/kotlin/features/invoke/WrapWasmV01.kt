package features.invoke

import io.polywrap.configBuilder.polywrapClient
import io.polywrap.core.InvokeResult
import io.polywrap.core.resolution.Uri
import io.polywrap.wasm.WasmPackage
import kotlinx.serialization.Serializable
import util.pathFromTemplate

@Serializable
data class WrapWasmV01Input(
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

fun wrapWasmV01(input: WrapWasmV01Input) {
    val wrapDir = pathFromTemplate(input.directory)
    val manifest = wrapDir.resolve("wrap.info").toFile().readBytes()
    val wasmModule = wrapDir.resolve("wrap.wasm").toFile().readBytes()
    val wrapPackage = WasmPackage(manifest, wasmModule)

    val client = polywrapClient {
        addPackage("embed/foo" to wrapPackage)
    }

    println("Invoking ${input.method}")

    val result: InvokeResult<Int> = client.invoke(
        uri = Uri.fromString("embed/foo"),
        method = input.method,
        args = input.args
    )

    if (result.isSuccess) {
        println("Received: ${result.getOrThrow()}")
        println("Success!")
    }
}