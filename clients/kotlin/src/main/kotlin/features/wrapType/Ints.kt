package features.wrapType

import io.polywrap.configBuilder.ConfigBuilder
import io.polywrap.core.resolution.Uri
import kotlinx.serialization.Contextual
import kotlinx.serialization.Serializable
import util.root

@Serializable
data class IntsInput(
    val method: String,
    val args: Map<String, @Contextual Any>
)

fun ints(input: IntsInput) {
    val root = root().resolve("wraps")
    val uri = "fs/$root/ints-type/implementations/as"

    val client = ConfigBuilder().addDefaults().build()

    println("Invoking ${input.method}")

    val response = client.invoke<Int>(
        uri = Uri.fromString(uri),
        method = input.method,
        args = input.args
    ).getOrThrow()

    println("Result: $response")
    println("Success!")
}