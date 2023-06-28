package features.wrapType

import io.polywrap.configBuilder.ConfigBuilder
import io.polywrap.core.resolution.Uri
import kotlinx.serialization.Contextual
import kotlinx.serialization.Serializable
import util.root

@Serializable
data class EnumInput(
    val method: String,
    val args: Map<String, @Contextual Any>
)

fun enumType(input: EnumInput) {
    val root = root().resolve("wraps")
    val uri = "fs/$root/enum-type/implementations/as"

    val client = ConfigBuilder().addDefaults().build()

    println("Invoking ${input.method}")

    val response = client.invoke<Int>(
        uri = Uri(uri),
        method = input.method,
        args = input.args
    ).getOrThrow()

    println("Result: $response")
    println("Success!")
}