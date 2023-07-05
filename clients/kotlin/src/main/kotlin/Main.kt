import features.config.*
import features.invoke.*
import features.resolve.*
import features.subinvoke.*
import features.uri.*
import features.wrapFeature.*
import features.wrapType.*
import util.*

fun main(args: Array<String>) {
    val filter: String? = if (args.isNotEmpty() && args[0].isNotEmpty()) args[0] else null
    val loader = SpecReader(root().resolve("specs"))

    val specs: List<String> = listOf(
        "config_embed_wrap_package",
        "config_env_variables",
        "config_interface_implementations",
        "config_plugin_instance",
        "config_plugin_package",
//        "config_resolver",
//        "config_resolver_ext",
//        "config_uri_redirect",
        "invoke_plugin",
        "invoke_wrap_wasm_v0_1",
//        "resolve_ens_contenthash",
//        "resolve_ens_text_record",
//        "resolve_file",
//        "resolve_http",
//        "resolve_instance",
//        "resolve_ipfs",
//        "resolve_package",
//        "resolve_redirect",
        "subinvoke_plugin_wrap",
        "subinvoke_wrap_plugin",
        "subinvoke_wrap_wrap",
        "uri",
        "wrap_feature_env_vars",
        "wrap_feature_interface_invoke",
        "wrap_type_bigint",
        "wrap_type_bignumber",
        "wrap_type_bytes",
        "wrap_type_enum",
        "wrap_type_ints",
        "wrap_type_json",
        "wrap_type_map",
        "wrap_type_object"
    )

    specs.forEach { name ->
        if (filter == null || filter.contains(name)) {
            val spec = loadSpec(name, loader)
            if (spec.required) runTest(name, spec)
        }
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
        "resolve_ens_contenthash" -> loader.readSpec<EnsContentHashInput>(name)
        "resolve_ens_text_record" -> loader.readSpec<EnsTextRecordInput>(name)
        "resolve_file" -> loader.readSpec<FileInput>(name)
        "resolve_http" -> loader.readSpec<HttpInput>(name)
        "resolve_instance" -> loader.readSpec<InstanceInput>(name)
        "resolve_ipfs" -> loader.readSpec<IpfsInput>(name)
        "resolve_package" -> loader.readSpec<PackageInput>(name)
        "resolve_redirect" -> loader.readSpec<RedirectInput>(name)
        "subinvoke_plugin_wrap" -> loader.readSpec<PluginWrapInput>(name)
        "subinvoke_wrap_plugin" -> loader.readSpec<WrapPluginInput>(name)
        "subinvoke_wrap_wrap" -> loader.readSpec<WrapWrapInput>(name)
        "uri" -> loader.readSpec<UriInput>(name)
        "wrap_feature_env_vars" -> loader.readSpec<EnvVarsInput>(name)
        "wrap_feature_interface_invoke" -> loader.readSpec<InterfaceInvokeInput>(name)
        "wrap_type_bigint" -> loader.readSpec<BigIntInput>(name)
        "wrap_type_bignumber" -> loader.readSpec<BigNumberInput>(name)
        "wrap_type_bytes" -> loader.readSpec<BytesInput>(name)
        "wrap_type_enum" -> loader.readSpec<EnumInput>(name)
        "wrap_type_ints" -> loader.readSpec<IntsInput>(name)
        "wrap_type_json" -> loader.readSpec<JsonInput>(name)
        "wrap_type_map" -> loader.readSpec<MapInput>(name)
        "wrap_type_object" -> loader.readSpec<ObjectInput>(name)
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
            System.err.println("!Test Error [$specName.$testName]")
            System.err.println(e)
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
        "invoke_plugin" -> runTestCase(name, spec, ::invokePlugin)
        "invoke_wrap_wasm_v0_1" -> runTestCase(name, spec, ::invokeWrapWasmV01)
        "resolve_ens_contenthash" -> runTestCase(name, spec, ::resolveEnsContentHash)
        "resolve_ens_text_record" -> runTestCase(name, spec, ::resolveEnsTextRecord)
        "resolve_file" -> runTestCase(name, spec, ::resolveFile)
        "resolve_http" -> runTestCase(name, spec, ::resolveHttp)
        "resolve_instance" -> runTestCase(name, spec, ::resolveInstance)
        "resolve_ipfs" -> runTestCase(name, spec, ::resolveIpfs)
        "resolve_package" -> runTestCase(name, spec, ::resolvePackage)
        "resolve_redirect" -> runTestCase(name, spec, ::resolveRedirect)
        "subinvoke_plugin_wrap" -> runTestCase(name, spec, ::pluginWrap)
        "subinvoke_wrap_plugin" -> runTestCase(name, spec, ::wrapPlugin)
        "subinvoke_wrap_wrap" -> runTestCase(name, spec, ::wrapWrap)
        "uri" -> runTestCase(name, spec, ::uri)
        "wrap_feature_env_vars" -> runTestCase(name, spec, ::envVarsFeature)
        "wrap_feature_interface_invoke" -> runTestCase(name, spec, ::interfaceInvokeFeature)
        "wrap_type_bigint" -> runTestCase(name, spec, ::bigIntType)
        "wrap_type_bignumber" -> runTestCase(name, spec, ::bigNumberType)
        "wrap_type_bytes" -> runTestCase(name, spec, ::bytesType)
        "wrap_type_enum" -> runTestCase(name, spec, ::enumType)
        "wrap_type_ints" -> runTestCase(name, spec, ::intsType)
        "wrap_type_json" -> runTestCase(name, spec, ::jsonType)
        "wrap_type_map" -> runTestCase(name, spec, ::mapType)
        "wrap_type_object" -> runTestCase(name, spec, ::objectType)
        else -> throw Exception("Spec not implemented: $name")
    }
}