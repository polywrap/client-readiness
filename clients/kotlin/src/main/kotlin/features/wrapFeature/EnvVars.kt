package features.wrapFeature

import io.polywrap.configBuilder.polywrapClient
import io.polywrap.core.InvokeResult
import io.polywrap.core.resolution.Uri
import kotlinx.serialization.Contextual
import kotlinx.serialization.Serializable
import util.root

@Serializable
data class EnvVarsInput(
    val mainEnv: Map<String, @Contextual Any>,
    val subinvokerEnv: Map<String, @Contextual Any>
)

fun envVars(input: EnvVarsInput) {
    val wrapsDir = root().resolve("wraps")
    val mainPath = wrapsDir.resolve("env-type/00-main/implementations/as")
    val mainUri = "file/${mainPath}"
    val subinvokerPath = wrapsDir.resolve("env-type/02-subinvoker-with-env/implementations/as")
    val subinvokerUri = "file/${subinvokerPath}"

    val envs = mapOf(
        mainUri to input.mainEnv,
        subinvokerUri to input.subinvokerEnv
    )

    val client = polywrapClient {
        addDefaults()
        addEnvs(envs)
        addRedirect("mock/main" to mainUri)
    }

    println("Invoking methodRequireEnv")

    val methodRequireEnvResult: InvokeResult<String> = client.invoke(
        uri = Uri.fromString(mainUri),
        method = "methodRequireEnv",
        args = mapOf("arg" to "string")
    )

    if (methodRequireEnvResult.isFailure) {
        throw methodRequireEnvResult.exceptionOrNull()!!
    }

    println("response.str: ${methodRequireEnvResult.getOrThrow()}")
    println("Success!")

    println("Invoking subinvokeMethodRequireEnv")

    val subinvokeEnvMethodResult: InvokeResult<String> = client.invoke(
        uri = Uri.fromString(subinvokerUri),
        method = "subinvokeMethodRequireEnv",
        args = mapOf("arg" to "string")
    )

    if (subinvokeEnvMethodResult.isFailure) {
        throw subinvokeEnvMethodResult.exceptionOrNull()!!
    }

    println("response.str: ${subinvokeEnvMethodResult.getOrThrow()}")
    println("Success!")
}