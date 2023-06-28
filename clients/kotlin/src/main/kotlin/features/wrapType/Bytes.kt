package features.wrapType

import io.polywrap.configBuilder.ConfigBuilder
import io.polywrap.core.InvokeResult
import io.polywrap.core.resolution.Uri
import kotlinx.serialization.Serializable
import util.root

@Serializable
data class BytesInput(
    val method: String,
    val args: BytesArgs
) {
    @Serializable
    data class BytesArgs(val arg: BytesList)
    @Serializable
    data class BytesList(val prop: ByteArray)
}

fun bytesType(input: BytesInput) {
    val root = root().resolve("wraps")
    val uri = "fs/$root/bytes-type/implementations/as"

    val client = ConfigBuilder().addDefaults().build()

    println("Invoking ${input.method}")

    val response: InvokeResult<ByteArray> = client.invoke(
        uri = Uri(uri),
        method = input.method,
        args = input.args
    )

    if (response.isFailure) {
        throw response.exceptionOrNull()!!
    } else {
        val bytes = response.getOrThrow().contentToString()
        println("Result: $bytes")
        println("Success!")
    }
}