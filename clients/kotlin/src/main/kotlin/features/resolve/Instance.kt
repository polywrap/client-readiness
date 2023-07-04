package features.resolve

import io.polywrap.configBuilder.polywrapClient
import io.polywrap.core.resolution.Uri
import io.polywrap.wasm.WasmWrapper
import kotlinx.serialization.Serializable
import util.pathFromTemplate

@Serializable
data class InstanceInput(
    val directory: String,
    val uri: String
)

fun resolveInstance(input: InstanceInput) {
    val wrapDir = pathFromTemplate(input.directory)
    val wasmModule = wrapDir.resolve("wrap.wasm").toFile().readBytes()
    val wrapper = WasmWrapper(wasmModule)

    val client = polywrapClient {
        setWrapper(input.uri to wrapper)
    }

    println("Resolving URI: ${input.uri}")

    val result = client.loadWrapper(Uri(input.uri))

    if (result.isSuccess) {
        println("Received: wrapper")
        println("Success!")
    }
}