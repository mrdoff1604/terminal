package dev.waylon.terminal.boundedcontexts.terminalsession.infrastructure.config

import dev.waylon.terminal.boundedcontexts.terminalsession.application.service.TerminalCommunicationService
import dev.waylon.terminal.boundedcontexts.terminalsession.infrastructure.protocol.WebSocketProtocol
import io.ktor.server.application.Application
import io.ktor.server.routing.routing
import io.ktor.server.websocket.webSocket
import io.ktor.websocket.CloseReason
import io.ktor.websocket.close
import org.koin.ktor.ext.inject

fun Application.configureTerminalWebSocketRoutes() {
    routing {
        webSocket("/ws/{sessionId}") { // websocketSession
            val sessionId = call.parameters["sessionId"] ?: return@webSocket close(
                CloseReason(
                    CloseReason.Codes.PROTOCOL_ERROR,
                    "Invalid session ID"
                )
            )

            val terminalCommunicationService by inject<TerminalCommunicationService>()

            val portocol = WebSocketProtocol(this)
            // Handle communication
            terminalCommunicationService.handleCommunication(sessionId, portocol)
        }
    }
}
