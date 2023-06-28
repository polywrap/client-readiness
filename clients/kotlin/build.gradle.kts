plugins {
    kotlin("jvm") version "1.8.22"
    kotlin("plugin.serialization") version "1.8.22"
    application
}

group = "io.polywrap"
version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
    maven {
        url = uri("https://s01.oss.sonatype.org/content/repositories/snapshots/")
    }
}

dependencies {
    testImplementation(kotlin("test"))
    implementation("io.polywrap:polywrap-client:0.10.0-SNAPSHOT")
    implementation("net.mamoe.yamlkt:yamlkt:0.13.0")
}

tasks.test {
    useJUnitPlatform()
}

kotlin {
    jvmToolchain(17)
}

application {
    mainClass.set("MainKt")
}

tasks.named<JavaExec>("run") {
    val appArgsStr: String? = project.findProperty("appArgs") as String?
    val appArgs = appArgsStr?.split(",") ?: emptyList()
    args(appArgs)
}
