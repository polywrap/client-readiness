package util

import kotlinx.serialization.decodeFromString
import net.mamoe.yamlkt.Yaml
import java.nio.file.Path

class Input(val specsDir: Path) {
    inline fun <reified T>readSpec(name: String): Spec<T> {
        val specStr = specsDir.resolve("$name.yaml").toFile().readText()
        return Yaml.decodeFromString(specStr)
    }
}