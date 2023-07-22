package mocks.incrementPlugin

import io.polywrap.core.Invoker
import io.polywrap.core.msgpack.msgPackEncode
import io.polywrap.plugin.PluginMethod
import io.polywrap.plugin.PluginModule
import kotlinx.serialization.Serializable
import kotlinx.serialization.serializer

class IncrementPlugin(config: Config? = null) : Module<IncrementPlugin.Config?>(config) {
    class Config()

    var counter: Int = 0

    override suspend fun increment(args: ArgsIncrement, invoker: Invoker) {
        counter++
    }
}

@Serializable
class ArgsIncrement

@Suppress("UNUSED_PARAMETER", "FunctionName")
abstract class Module<TConfig>(config: TConfig) : PluginModule<TConfig>(config) {

    final override val methods: Map<String, PluginMethod> = mapOf(
        "increment" to ::__increment,
    )

    abstract suspend fun increment(
        args: ArgsIncrement,
        invoker: Invoker
    )

    private suspend fun __increment(
        encodedArgs: ByteArray?,
        encodedEnv: ByteArray?,
        invoker: Invoker
    ): ByteArray {
        val args = ArgsIncrement()
        val response = increment(args, invoker)
        return msgPackEncode(serializer(), response)
    }
}