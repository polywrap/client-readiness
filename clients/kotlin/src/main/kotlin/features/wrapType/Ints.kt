package features.wrapType

import io.polywrap.configBuilder.ConfigBuilder
import io.polywrap.core.InvokeResult
import io.polywrap.core.resolution.Uri
import kotlinx.serialization.Serializable
import util.root

@Serializable
data class IntsInput(
    val method: String,
    val args: IntArgs
) {
    @Serializable
    data class IntArgs(val first: Int, val second: Int)
}

fun intsType(input: IntsInput) {
    val root = root().resolve("wraps")
    val uri = "fs/$root/numbers-type/implementations/as"

    val client = ConfigBuilder().addDefaults().build()

    println("Invoking ${input.method}")

    val response: InvokeResult<Int> = client.invoke(
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