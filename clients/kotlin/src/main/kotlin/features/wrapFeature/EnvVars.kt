package features.wrapFeature

import io.polywrap.configBuilder.polywrapClient
import io.polywrap.core.resolution.Uri
import kotlinx.serialization.Contextual
import kotlinx.serialization.Serializable
import util.root

@Serializable
data class EnvVarsInput(
    val mainEnv: Map<String, @Contextual Any>,
    val subinvokerEnv: Map<String, @Contextual Any>
)

fun envVarsFeature(input: EnvVarsInput) {
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
        setRedirect("mock/main" to mainUri)
    }

    println("Invoking methodRequireEnv")

    val methodRequireEnvResult: Map<String, Any> = client.invoke<Map<String, Any>>(
        uri = Uri(mainUri),
        method = "methodRequireEnv",
        args = mapOf("arg" to "string")
    ).getOrThrow()

    println("response.str: ${methodRequireEnvResult["str"]}")
    println("Success!")

    println("Invoking subinvokeMethodRequireEnv")

    val subinvokeEnvMethodResult: Map<String, Any> = client.invoke<Map<String, Any>>(
        uri = Uri(subinvokerUri),
        method = "subinvokeMethodRequireEnv",
        args = mapOf("arg" to "string")
    ).getOrThrow()

    println("response.str: ${subinvokeEnvMethodResult["str"]}")
    println("Success!")
}