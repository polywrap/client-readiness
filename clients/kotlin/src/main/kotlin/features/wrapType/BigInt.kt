package features.wrapType

import io.polywrap.configBuilder.ConfigBuilder
import io.polywrap.core.InvokeResult
import io.polywrap.core.resolution.Uri
import kotlinx.serialization.Serializable
import util.root

@Serializable
data class BigIntInput (
    val args: MethodInput
) {
    @Serializable
    data class MethodInput(val arg1: String, val obj: MethodObj)

    @Serializable
    data class MethodObj(val prop1: String)
}

fun bigInt(input: BigIntInput) {
    val root = root().resolve("wraps")
    val uri = "fs/$root/bigint-type/implementations/as"

    val client = ConfigBuilder().addDefaults().build()

    println("Invoking method")

    val response: InvokeResult<String> = client.invoke(
        uri = Uri.fromString(uri),
        method = "method",
        args = input.args
    )

    println("Result: ${response.getOrThrow()}")
    println("Success!")
}