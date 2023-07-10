package features.wrapType

import io.polywrap.configBuilder.ConfigBuilder
import io.polywrap.core.InvokeResult
import io.polywrap.core.resolution.Uri
import kotlinx.serialization.Serializable
import util.root

@Serializable
data class EnumInput(
    val method: String,
    val args: EnumArgs
) {
    @Serializable
    data class EnumArgs(val en: String)

    @Serializable
    data class NumEnumArgs(val en: Int)
}

fun enumType(input: EnumInput) {
    val root = root().resolve("wraps")
    val uri = "fs/$root/enum-type/implementations/as"

    val client = ConfigBuilder().addDefaults().build()

    println("Invoking ${input.method}")

    val numeric = input.args.en.toIntOrNull()
    val response: InvokeResult<Int> = if (numeric != null) {
        client.invoke(
            uri = Uri(uri),
            method = input.method,
            args = EnumInput.NumEnumArgs(numeric)
        )
    } else {
        client.invoke(
            uri = Uri(uri),
            method = input.method,
            args = input.args
        )
    }

    if (response.isFailure) {
        throw response.exceptionOrNull()!!
    } else {
        println("Result: ${response.getOrThrow()}")
        println("Success!")
    }
}