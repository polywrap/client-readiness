import java.io.FileOutputStream

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
    implementation("com.ensarsarajcic.kotlinx:serialization-msgpack:0.5.5") // not sure why but this is needed
    implementation("net.mamoe.yamlkt:yamlkt:0.13.0")
    implementation("org.slf4j:slf4j-nop:1.7.36") // suppress SLF4J logger warnings
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
    project.findProperty("appArgs")?.let { args(it) }
    standardOutput = FileOutputStream("$projectDir/stdout")
    errorOutput = FileOutputStream("$projectDir/stderr")
}
