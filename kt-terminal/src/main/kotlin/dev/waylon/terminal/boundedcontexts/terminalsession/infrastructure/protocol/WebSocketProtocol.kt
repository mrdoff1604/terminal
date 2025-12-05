package dev.waylon.terminal.boundedcontexts.terminalsession.infrastructure.protocol

import dev.waylon.terminal.boundedcontexts.terminalsession.application.TerminalCommunicationProtocol
import io.ktor.server.websocket.DefaultWebSocketServerSession
import io.ktor.websocket.CloseReason
import io.ktor.websocket.Frame
import io.ktor.websocket.close
import io.ktor.websocket.readText
import kotlinx.coroutines.channels.ClosedReceiveChannelException
import kotlinx.coroutines.channels.ClosedSendChannelException

/**
 * WebSocket implementation of TerminalCommunicationProtocol
 */
class WebSocketProtocol(
    private val session: DefaultWebSocketServerSession
) : TerminalCommunicationProtocol {
    private val log = org.slf4j.LoggerFactory.getLogger(WebSocketProtocol::class.java)

    override suspend fun send(data: String) {
        try {
            log.debug("Sending data to WebSocket client: {}", data)
            session.outgoing.send(Frame.Text(data))
        } catch (e: ClosedSendChannelException) {
            // Client disconnected
            log.debug("WebSocket client disconnected, cannot send data: {}", e.message)
            throw e
        } catch (e: Exception) {
            log.error("Error sending data to WebSocket client: {}", e.message, e)
            throw e
        }
    }

    override suspend fun receive(): String? {
        return try {
            when (val frame = session.incoming.receive()) {
                is Frame.Text -> {
                    val text = frame.readText()
                    log.debug("Received data from WebSocket client: {}", text)
                    text
                }

                is Frame.Close -> {
                    log.debug("Received close frame from WebSocket client")
                    null
                }

                else -> {
                    log.debug("Received non-text frame from WebSocket client: {}", frame.frameType)
                    null
                }
            }
        } catch (_: ClosedReceiveChannelException) {
            // Client disconnected
            log.debug("WebSocket client disconnected, closing receive loop")
            null
        } catch (e: Exception) {
            log.error("Error receiving data from WebSocket client: {}", e.message, e)
            null
        }
    }

    override suspend fun close(reason: String?) {
        val closeReason = reason ?: "Connection closed"
        log.debug("Closing WebSocket connection: {}", closeReason)
        session.close(CloseReason(CloseReason.Codes.GOING_AWAY, closeReason))
    }
}
