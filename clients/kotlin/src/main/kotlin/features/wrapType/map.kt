package features.wrapType

import io.polywrap.configBuilder.ConfigBuilder
import io.polywrap.core.msgpack.MsgPackMap
import io.polywrap.core.msgpack.toMsgPackMap
import io.polywrap.core.resolution.Uri
import kotlinx.serialization.Serializable
import util.root

@Serializable
data class MapInput(
    val map: Map<String, Int>
)

@Serializable
data class ArgsReturnMap(val map: MsgPackMap<String, Int>)

fun mapType(input: MapInput) {
    val root = root().resolve("wraps")
    val uri = "fs/$root/map-type/implementations/as"

    val client = ConfigBuilder().addDefaults().build()

    println("Invoking method")

    val response = client.invoke<ArgsReturnMap, MsgPackMap<String, Int>>(
        uri = Uri(uri),
        method = "returnMap",
        args = ArgsReturnMap(input.map.toMsgPackMap())
    ).getOrThrow()

    response.map.forEach {
        println("key '${it.key}' = ${it.value}")
    }

    println("Success!")
}
