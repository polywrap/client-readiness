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

fun pathStringFromTemplate(template: String): String {
    if (template.contains("\$ROOT")) {
        val root = root().toString()
        return template.replace("\$ROOT", root)
    }
    return template
}

fun pathFromTemplate(template: String): Path {
    val updated = pathStringFromTemplate(template)
    return Path(updated)
}

fun uriAuthority(uri: String): String {
    return uri.substring("wrap://".length).split("/")[0]
}

fun uriPath(uri: String): String {
    return uri.substring("wrap://".length + uriAuthority(uri).length + 1)
}