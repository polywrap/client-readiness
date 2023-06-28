package features.wrapType

import io.polywrap.configBuilder.ConfigBuilder
import io.polywrap.core.InvokeResult
import io.polywrap.core.resolution.Uri
import kotlinx.serialization.Serializable
import util.root

@Serializable
data class ObjectInput(
    val method: String,
    val args: ObjectArgs
) {
    @Serializable
    data class ObjectArgs(val arg1: Output)
}

@Serializable
data class Output(val prop: String, val nested: Nested) {
    @Serializable
    data class Nested(val prop: String)
}

fun objectType(input: ObjectInput) {
    val root = root().resolve("wraps")
    val uri = "fs/$root/object-type/implementations/as"

    val client = ConfigBuilder().addDefaults().build()

    println("Invoking ${input.method}")

    val response: InvokeResult<List<Output>> = client.invoke(
        uri = Uri(uri),
        method = input.method,
        args = input.args
    )

    if (response.isFailure) {
        throw response.exceptionOrNull()!!
    } else {
        println("Result: ${response.getOrThrow()}")
        println("Success!")
    }
}