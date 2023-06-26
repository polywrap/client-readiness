package features.wrapType

import io.polywrap.configBuilder.ConfigBuilder
import io.polywrap.core.resolution.Uri
import kotlinx.serialization.Contextual
import kotlinx.serialization.Serializable
import util.root

@Serializable
data class BigIntInput (
    val args: Map<String, @Contextual Any>
)

fun bigInt(input: BigIntInput) {
    val root = root().resolve("wraps")
    val uri = "fs/$root/bigint-type/implementations/as"

    val client = ConfigBuilder().addDefaults().build()

    println("Invoking method")

    val response = client.invoke<String>(
        uri = Uri.fromString(uri),
        method = "method",
        args = input.args
    ).getOrThrow()

    println("Result: $response")
    println("Success!")
}