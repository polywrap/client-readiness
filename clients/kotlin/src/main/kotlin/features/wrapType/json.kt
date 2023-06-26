package features.wrapType

import io.polywrap.configBuilder.ConfigBuilder
import io.polywrap.core.resolution.Uri
import kotlinx.serialization.Contextual
import kotlinx.serialization.Serializable
import util.root

@Serializable
data class JsonInput(
    val method: String,
    val args: Map<String, @Contextual Any>
)

fun json(input: JsonInput) {
    val root = root().resolve("wraps")
    val uri = "fs/$root/json-type/implementations/as"

    val client = ConfigBuilder().addDefaults().build()

    println("Invoking ${input.method}")

    val response = client.invoke<String>(
        uri = Uri.fromString(uri),
        method = input.method,
        args = input.args
    ).getOrThrow()

    println("Result: $response")
    println("Success!")
}