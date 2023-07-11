package mocks.addPlugin

import io.polywrap.core.Invoker
import io.polywrap.core.msgpack.msgPackDecode
import io.polywrap.core.msgpack.msgPackEncode
import io.polywrap.plugin.PluginFactory
import io.polywrap.plugin.PluginMethod
import io.polywrap.plugin.PluginModule
import io.polywrap.plugin.PluginPackage
import kotlinx.serialization.Serializable
import kotlinx.serialization.serializer
import mocks.mockManifest

val addPlugin: PluginFactory<AddPlugin.Config?> = { config: AddPlugin.Config? ->
    PluginPackage(
        pluginModule = AddPlugin(config),
        manifest = mockManifest
    )
}

class AddPlugin(config: Config? = null) : Module<AddPlugin.Config?>(config) {
    class Config()

    override suspend fun add(args: ArgsAdd, invoker: Invoker): Int {
        return args.a + args.b
    }

    override suspend fun concat(args: ArgsConcat, invoker: Invoker): String {
        return args.a + args.b
    }
}

@Serializable
data class ArgsAdd(
    val a: Int,
    val b: Int
)

@Serializable
data class ArgsConcat(
    val a: String,
    val b: String
)

@Suppress("UNUSED_PARAMETER", "FunctionName")
abstract class Module<TConfig>(config: TConfig) : PluginModule<TConfig>(config) {

    final override val methods: Map<String, PluginMethod> = mapOf(
        "add" to ::__add,
        "concat" to ::__concat
    )

    abstract suspend fun add(
        args: ArgsAdd,
        invoker: Invoker
    ): Int

    abstract suspend fun concat(
        args: ArgsConcat,
        invoker: Invoker
    ): String

    private suspend fun __add(
        encodedArgs: ByteArray?,
        encodedEnv: ByteArray?,
        invoker: Invoker
    ): ByteArray {
        val args: ArgsAdd = encodedArgs?.let { msgPackDecode(serializer<ArgsAdd>(), it).getOrNull() }
            ?: throw Error("Missing args in invocation to plugin method 'add'")
        val response = add(args, invoker)
        return msgPackEncode(serializer(), response)
    }

    private suspend fun __concat(
        encodedArgs: ByteArray?,
        encodedEnv: ByteArray?,
        invoker: Invoker
    ): ByteArray {
        val args: ArgsConcat = encodedArgs?.let { msgPackDecode(serializer<ArgsConcat>(), it).getOrNull() }
            ?: throw Error("Missing args in invocation to plugin method 'concat'")
        val response = concat(args, invoker)
        return msgPackEncode(serializer(), response)
    }
}
