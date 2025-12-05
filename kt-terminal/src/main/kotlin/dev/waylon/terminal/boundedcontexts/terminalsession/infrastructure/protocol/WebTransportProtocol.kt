package dev.waylon.terminal.boundedcontexts.terminalsession.infrastructure.protocol

import dev.waylon.terminal.boundedcontexts.terminalsession.application.TerminalCommunicationProtocol

/**
 * WebTransport implementation of TerminalCommunicationProtocol
 * This is a placeholder implementation until Ktor supports WebTransport
 *
 * Note: Currently Ktor does not support WebTransport. This implementation
 * is provided as a placeholder to demonstrate how WebTransport support
 * could be added once Ktor supports it.
 *
 * When Ktor adds WebTransport support, this class will be updated to use
 * the actual WebTransport API, likely with a constructor parameter similar
 * to WebSocketProtocol's DefaultWebSocketServerSession.
 */
class WebTransportProtocol : TerminalCommunicationProtocol {
    override suspend fun send(data: String) {
        throw UnsupportedOperationException("WebTransport is not supported yet")
    }

    override suspend fun receive(): String? {
        throw UnsupportedOperationException("WebTransport is not supported yet")
    }

    override suspend fun close(reason: String?) {
        // No-op for now
    }
}
