package mocks.subinvokePlugin

import io.polywrap.core.Invoker
import io.polywrap.core.msgpack.msgPackDecode
import io.polywrap.core.msgpack.msgPackEncode
import io.polywrap.core.resolution.Uri
import io.polywrap.plugin.PluginFactory
import io.polywrap.plugin.PluginMethod
import io.polywrap.plugin.PluginModule
import io.polywrap.plugin.PluginPackage
import kotlinx.serialization.Contextual
import kotlinx.serialization.Serializable
import kotlinx.serialization.serializer
import mocks.mockManifest

val subinvokePlugin: PluginFactory<SubinvokePlugin.Config?> = { config: SubinvokePlugin.Config? ->
    PluginPackage(
        pluginModule = SubinvokePlugin(config),
        manifest = mockManifest
    )
}

class SubinvokePlugin(config: Config? = null) : Module<SubinvokePlugin.Config?>(config) {
    class Config()

    override suspend fun performSubinvoke(args: ArgsSubinvoke, invoker: Invoker): Int {
        println("Subinvoking ${args.method}")
        val res: Int? = invoker.invoke<Int>(Uri.fromString(args.uri), args.method, args.args).getOrNull()
        return if (res != null) {
            println("Received: $res")
            res
        } else {
            -1
        }
    }
}

@Serializable
data class ArgsSubinvoke(
    val uri: String,
    val method: String,
    val args: Map<String, @Contextual Any>
)

@Suppress("UNUSED_PARAMETER", "FunctionName")
abstract class Module<TConfig>(config: TConfig) : PluginModule<TConfig>(config) {

    final override val methods: Map<String, PluginMethod> = mapOf(
        "performSubinvoke" to ::__performSubinvoke,
    )

    abstract suspend fun performSubinvoke(
        args: ArgsSubinvoke,
        invoker: Invoker
    ): Int

    private suspend fun __performSubinvoke(
        encodedArgs: ByteArray?,
        encodedEnv: ByteArray?,
        invoker: Invoker
    ): ByteArray {
        val args: ArgsSubinvoke = encodedArgs?.let { msgPackDecode(serializer<ArgsSubinvoke>(), it).getOrNull() }
            ?: throw Error("Missing args in invocation to plugin method 'performSubinvoke'")
        val response = performSubinvoke(args, invoker)
        return msgPackEncode(serializer(), response)
    }
}
