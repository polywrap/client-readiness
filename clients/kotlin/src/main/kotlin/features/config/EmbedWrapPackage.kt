package features.config

import io.polywrap.configBuilder.polywrapClient
import io.polywrap.core.InvokeResult
import io.polywrap.core.resolution.Uri
import io.polywrap.wasm.WasmPackage
import kotlinx.serialization.Serializable
import util.pathFromTemplate

@Serializable
data class EmbedWrapPackageInput(
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

fun embedWrapPackage(input: EmbedWrapPackageInput) {
    val wrapDir = pathFromTemplate(input.directory)

    println("Reading wrap.info & wrap.wasm from ${input.directory}")

    val manifest = wrapDir.resolve("wrap.info").toFile().readBytes()
    val wasmModule = wrapDir.resolve("wrap.wasm").toFile().readBytes()

    println("Creating WrapPackage from raw wrap.info & wrap.wasm bytes")

    val wrapPackage = WasmPackage(manifest, wasmModule)

    println("Adding WrapPackage to ClientConfig")

    val client = polywrapClient {
        setPackage("embed/foo" to wrapPackage)
    }

    println("Invoking WrapPackage")

    val result: InvokeResult<Int> = client.invoke(
        uri = Uri("embed/foo"),
        method = input.method,
        args = input.args
    )

    if (result.isSuccess) {
        println("Success!")
    }
}
