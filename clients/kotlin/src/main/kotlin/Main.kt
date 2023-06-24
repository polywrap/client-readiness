import features.config.EmbedWrapPackageInput
import features.config.embedWrapPackage
import util.Input
import util.Spec
import util.root

fun main(args: Array<String>) {
    val filter: String? = if (args.size > 2) args[2] else null
    val input = Input(root().resolve("specs"))

    val configEmbedWrapPackageName = "config_embed_wrap_package"
    val configEmbedWrapPackageSpec: Spec<EmbedWrapPackageInput> = input.readSpec(configEmbedWrapPackageName)

    val specs: Map<String, Spec<*>> = mapOf(
        configEmbedWrapPackageName to configEmbedWrapPackageSpec
    )

    specs.forEach { (name, spec) ->
        if (filter != name && spec.required) runTestCase(name, spec, ::embedWrapPackage)
    }
}

inline fun <reified K> runTestCase(specName: String, spec: Spec<*>, fn: (input: K) -> Unit) {
    println("====================================")
    println("Begin Feature Spec Test Cases [$specName]")
    println("====================================")

    for ((testName, case) in spec.cases) {
        println("\$Test Start [$specName.$testName]")
        try {
            fn(case.input as K)
        } catch(e: Exception) {
            println("!Test Error [$specName.$testName]")
            println(e)
        }
    }

    println("====================================")
    println("End Feature Spec Test Cases [$specName]")
    println("====================================")
}

