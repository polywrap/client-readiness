package features.wrapType

import io.polywrap.configBuilder.ConfigBuilder
import io.polywrap.core.resolution.Uri
import kotlinx.serialization.Contextual
import kotlinx.serialization.Serializable
import util.root

@Serializable
data class ObjectInput(
    val method: String,
    val args: Map<String, @Contextual Any>
)

@Serializable
private data class Output(val prop: String, val nested: Nested) {
    @Serializable
    data class Nested(val prop: String)
}

fun objectType(input: ObjectInput) {
    val root = root().resolve("wraps")
    val uri = "fs/$root/object-type/implementations/as"

    val client = ConfigBuilder().addDefaults().build()

    println("Invoking ${input.method}")

    val response = client.invoke<List<Output>>(
        uri = Uri(uri),
        method = input.method,
        args = input.args
    ).getOrThrow()

    println("Result: $response")
    println("Success!")
}