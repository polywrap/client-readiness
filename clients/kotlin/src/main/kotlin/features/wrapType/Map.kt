package features.wrapType

import io.polywrap.configBuilder.ConfigBuilder
import io.polywrap.core.msgpack.GenericMap
import io.polywrap.core.msgpack.toGenericMap
import io.polywrap.core.resolution.Uri
import kotlinx.serialization.Serializable
import util.root

@Serializable
data class MapInput(
    val map: Map<String, Int>
)

@Serializable
data class ArgsReturnMap(val map: GenericMap<String, Int>)

fun mapType(input: MapInput) {
    val root = root().resolve("wraps")
    val uri = "fs/$root/map-type/implementations/as"

    val client = ConfigBuilder().addDefaults().build()

    println("Invoking returnMap")

    val response = client.invoke<ArgsReturnMap, GenericMap<String, Int>>(
        uri = Uri(uri),
        method = "returnMap",
        args = ArgsReturnMap(input.map.toGenericMap())
    ).getOrThrow()

    response.map.forEach {
        println("key '${it.key}' = ${it.value}")
    }

    println("Success!")
}
