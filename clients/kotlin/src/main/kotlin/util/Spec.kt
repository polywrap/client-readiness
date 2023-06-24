package util

import kotlinx.serialization.Serializable

@Serializable
data class Spec<T>(val required: Boolean, val cases: Map<String, TestCase<T>>)

@Serializable
data class TestCase<T>(val input: T, val output: Output)

@Serializable
data class Output(val stdout: List<String>, val stderr: List<String>? = null)