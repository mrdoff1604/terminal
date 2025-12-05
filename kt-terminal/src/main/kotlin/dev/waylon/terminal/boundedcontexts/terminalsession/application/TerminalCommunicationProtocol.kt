package dev.waylon.terminal.boundedcontexts.terminalsession.application

interface TerminalCommunicationProtocol {
    /**
     * Send data to client
     */
    suspend fun send(data: String)

    /**
     * Receive data from client
     */
    suspend fun receive(): String?

    /**
     * Close the connection
     */
    suspend fun close(reason: String? = null)
}