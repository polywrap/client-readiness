package features.wrapFeature

import io.polywrap.configBuilder.polywrapClient
import io.polywrap.core.InvokeResult
import io.polywrap.core.resolution.Uri
import util.root

typealias InterfaceInvokeInput = Boolean

fun interfaceInvokeFeature(input: InterfaceInvokeInput) {
    val root = root().resolve("wraps")
    val interfaceUri = "wrap://ens/interface.eth"
    val implementationPath = root.resolve("interface-invoke/01-implementation/implementations/as")
    val implementationUri = "fs/$implementationPath"

    val client = polywrapClient {
        addDefaults()
        addInterfaceImplementation(interfaceUri, implementationUri)
    }

    val wrapperPath = root.resolve("interface-invoke/02-wrapper/implementations/as")
    val wrapperUri = "fs/$wrapperPath"

    println("Invoking moduleMethod")

    val result: InvokeResult<Unit> = client.invoke(
        uri = Uri.fromString(wrapperUri),
        method = "moduleMethod",
        args = mapOf(
            "arg" to mapOf(
                "uint8" to 1,
                "str" to "Test String 1"
            )
        )
    )

    if (result.isFailure) throw result.exceptionOrNull()!!

    println("Success!")
}