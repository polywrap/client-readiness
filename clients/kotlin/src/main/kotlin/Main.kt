import features.config.*
import features.invoke.*
import features.subinvoke.*
import util.*

fun main(args: Array<String>) {
    val filter: String? = if (args.size > 2) args[2] else null
    val loader = SpecReader(root().resolve("specs"))

    val specs: List<String> = listOf(
        "config_embed_wrap_package",
        "config_env_variables",
        "config_interface_implementations",
        "config_plugin_instance",
        "config_plugin_package",
        "config_resolver",
        "config_resolver_ext",
        "config_uri_redirect",
        "invoke_plugin",
        "invoke_wrap_wasm_v0_1",
        "subinvoke_plugin_wrap",
        "subinvoke_wrap_plugin",
        "subinvoke_wrap_wrap"
    )

    specs.forEach { name ->
        if (filter == name) return
        val spec = loadSpec(name, loader)
        if (spec.required) runTest(name, spec)
    }
}

fun loadSpec(name: String, loader: SpecReader): Spec<*> {
    return when (name) {
        "config_embed_wrap_package" -> loader.readSpec<EmbedWrapPackageInput>(name)
        "config_env_variables" -> loader.readSpec<EnvVariablesInput>(name)
        "config_interface_implementations" -> loader.readSpec<InterfaceImplementationsInput>(name)
        "config_plugin_instance" -> loader.readSpec<PluginInstanceInput>(name)
        "config_plugin_package" -> loader.readSpec<PluginPackageInput>(name)
        "config_resolver" -> loader.readSpec<ResolverInput>(name)
        "config_resolver_ext" -> loader.readSpec<ResolverExtInput>(name)
        "config_uri_redirect" -> loader.readSpec<UriRedirectInput>(name)
        "invoke_plugin" -> loader.readSpec<PluginInput>(name)
        "invoke_wrap_wasm_v0_1" -> loader.readSpec<WrapWasmV01Input>(name)
        "subinvoke_plugin_wrap" -> loader.readSpec<PluginWrapInput>(name)
        "subinvoke_wrap_plugin" -> loader.readSpec<WrapPluginInput>(name)
        "subinvoke_wrap_wrap" -> loader.readSpec<WrapWrapInput>(name)
        else -> throw Exception("Spec not implemented: $name")
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

fun runTest(name: String, spec: Spec<*>) {
    when (name) {
        "config_embed_wrap_package" -> runTestCase(name, spec, ::embedWrapPackage)
        "config_env_variables" -> runTestCase(name, spec, ::envVariables)
        "config_interface_implementations" -> runTestCase(name, spec, ::interfaceImplementations)
        "config_plugin_instance" -> runTestCase(name, spec, ::pluginInstance)
        "config_plugin_package" -> runTestCase(name, spec, ::pluginPackage)
        "config_resolver" -> runTestCase(name, spec, ::resolver)
        "config_resolver_ext" -> runTestCase(name, spec, ::resolverExt)
        "config_uri_redirect" -> runTestCase(name, spec, ::uriRedirect)
        "invoke_plugin" -> runTestCase(name, spec, ::plugin)
        "invoke_wrap_wasm_v0_1" -> runTestCase(name, spec, ::wrapWasmV01)
        "subinvoke_plugin_wrap" -> runTestCase(name, spec, ::pluginWrap)
        "subinvoke_wrap_plugin" -> runTestCase(name, spec, ::wrapPlugin)
        "subinvoke_wrap_wrap" -> runTestCase(name, spec, ::wrapWrap)
        else -> throw Exception("Spec not implemented: $name")
    }
}