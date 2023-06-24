package util

import java.nio.file.Path
import kotlin.io.path.Path

fun cwd(): Path = Path(System.getProperty("user.dir"))

fun root(): Path {
    val parts = cwd().joinToString("/").split("/")
    val index = parts.lastIndexOf("client-readiness")
    val root = "/" + parts.subList(0, index + 1).joinToString("/")
    return Path(root)
}

fun pathFromTemplate(template: String): Path {
    if (template.contains("\$ROOT")) {
        val root = root().toString()
        val path = template.replace("\$ROOT", root)
        return Path(path)
    }
    return Path(template)
}