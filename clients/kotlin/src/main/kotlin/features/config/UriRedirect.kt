package features.config

import io.polywrap.configBuilder.polywrapClient
import kotlinx.serialization.Serializable

@Serializable
data class UriRedirectInput(
    val from: String,
    val to: String
)

fun uriRedirect(input: UriRedirectInput) {
    println("Adding URI Redirect to ClientConfig")

    polywrapClient {
        setRedirect(input.from to input.to)
    }

    println("Success!")
}