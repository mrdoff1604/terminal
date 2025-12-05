package dev.waylon.terminal.boundedcontexts.terminalsession.infrastructure.config

import io.ktor.http.HttpStatusCode
import io.ktor.server.application.Application
import io.ktor.server.response.respondText
import io.ktor.server.routing.get
import io.ktor.server.routing.routing

/**
 * WebTransport routes for terminal communication
 *
 * Note: Currently Ktor does not support WebTransport. This implementation
 * provides a placeholder that will be updated once Ktor supports WebTransport.
 */
fun Application.configureTerminalWebTransportRoutes() {
    routing {
        // WebTransport support placeholder
        // When Ktor adds WebTransport support, we'll replace this with actual implementation
        get("/webtransport/{sessionId}") {
            call.respondText(
                text = "WebTransport is not supported yet. Please use WebSocket instead.",
                status = HttpStatusCode.NotImplemented
            )
        }


    }
}
