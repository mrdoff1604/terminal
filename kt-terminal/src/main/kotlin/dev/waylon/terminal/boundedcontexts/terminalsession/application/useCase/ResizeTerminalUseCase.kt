package dev.waylon.terminal.boundedcontexts.terminalsession.application.useCase

import dev.waylon.terminal.boundedcontexts.terminalsession.application.service.TerminalSessionService
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSession
import dev.waylon.terminal.boundedcontexts.terminalsession.infrastructure.dto.ResizeTerminalRequest
import org.slf4j.LoggerFactory

/**
 * Resize Terminal Use Case
 * Encapsulates business logic for resizing terminal, keeping the Route layer lightweight
 */
class ResizeTerminalUseCase(
    private val terminalSessionService: TerminalSessionService
) : UseCase<ResizeTerminalInput, TerminalSession> {
    private val log = LoggerFactory.getLogger(ResizeTerminalUseCase::class.java)

    /**
     * Execute resize terminal operation
     * @param input Resize terminal input data
     * @return Resized terminal session
     * @throws IllegalArgumentException If request parameters are invalid
     * @throws TerminalSessionNotFoundException If session not found
     */
    override suspend operator fun invoke(input: ResizeTerminalInput): TerminalSession {
        val sessionId = input.sessionId
        val request = input.request
        log.debug("Executing ResizeTerminalUseCase for sessionId: {}, request: {}", sessionId, request)

        // Validate request parameters
        validateResizeTerminalRequest(request)

        val session = terminalSessionService.resizeTerminal(sessionId, request.columns, request.rows)

        if (session != null) {
            log.debug("Resized terminal session {} successfully", sessionId)
        } else {
            log.debug("Failed to resize terminal session {}", sessionId)
        }

        return session
    }

    /**
     * Validate resize terminal request parameters
     * @param request Resize terminal request object
     * @throws IllegalArgumentException If request parameters are invalid
     */
    private fun validateResizeTerminalRequest(request: ResizeTerminalRequest) {
        // Validate columns must be positive
        require(request.columns > 0) { "columns must be greater than 0" }

        // Validate rows must be positive
        require(request.rows > 0) { "rows must be greater than 0" }

        // Can add more business rule validations, such as maximum size limits
        // require(request.columns <= 1000) { "columns must be less than or equal to 1000" }
        // require(request.rows <= 1000) { "rows must be less than or equal to 1000" }
    }
}
