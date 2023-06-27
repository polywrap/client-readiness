package features.wrapType

import io.polywrap.configBuilder.ConfigBuilder
import io.polywrap.core.resolution.Uri
import kotlinx.serialization.Contextual
import kotlinx.serialization.Serializable
import util.root

@Serializable
data class BytesInput(
    val method: String,
    val args: Map<String, @Contextual Any>
)

fun bytesType(input: BytesInput) {
    val root = root().resolve("wraps")
    val uri = "fs/$root/bytes-type/implementations/as"

    val client = ConfigBuilder().addDefaults().build()

    println("Invoking ${input.method}")

    val response = client.invoke<ByteArray>(
        uri = Uri.fromString(uri),
        method = input.method,
        args = input.args
    ).getOrThrow()

    println("Result: ${response.contentToString()}")
    println("Success!")
}