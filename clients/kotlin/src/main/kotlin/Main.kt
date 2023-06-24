import features.config.EmbedWrapPackageInput
import util.Input
import util.Spec
import util.root

fun main(args: Array<String>) {
    val input = Input(root().resolve("specs"))

    val configEmbedWrapPackage: Spec<EmbedWrapPackageInput> = input.readSpec("config_embed_wrap_package")

    println(configEmbedWrapPackage)
}

