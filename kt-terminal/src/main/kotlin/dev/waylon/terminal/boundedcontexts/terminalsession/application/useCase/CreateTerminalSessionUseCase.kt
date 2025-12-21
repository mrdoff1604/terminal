package dev.waylon.terminal.boundedcontexts.terminalsession.application.useCase

import dev.waylon.terminal.boundedcontexts.terminalsession.application.service.TerminalSessionService
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSession
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSize
import dev.waylon.terminal.boundedcontexts.terminalsession.infrastructure.dto.CreateSessionRequest
import org.slf4j.LoggerFactory

/**
 * Create Terminal Session Use Case
 * Encapsulates business logic for creating terminal sessions, keeping the Route layer lightweight
 */
class CreateTerminalSessionUseCase(
    private val terminalSessionService: TerminalSessionService
) : UseCase<CreateSessionRequest, TerminalSession> {
    private val log = LoggerFactory.getLogger(CreateTerminalSessionUseCase::class.java)

    /**
     * Execute create terminal session operation asynchronously
     * @param request Create session request object
     * @return Created terminal session
     * @throws IllegalArgumentException If request parameters are invalid
     */
    override suspend operator fun invoke(request: CreateSessionRequest): TerminalSession {
        log.debug("Executing CreateTerminalSessionUseCase")
        log.debug("Session creation request: $request")

        // Validate request parameters
        validateCreateSessionRequest(request)

        // Parse terminal size
        val terminalSize = if (request.columns != null && request.rows != null) {
            TerminalSize(request.columns, request.rows)
        } else {
            TerminalSize(80, 24) // Default size
        }

        // Call service layer to create session asynchronously
        val session = terminalSessionService.createSession(
            userId = request.userId,
            title = request.title,
            shellType = request.shellType,
            workingDirectory = request.workingDirectory,
            size = terminalSize
        )

        log.info(
            "Created new terminal session: {}, shellType: {}, workingDirectory: {}",
            session.id, session.shellType, session.workingDirectory
        )

        return session
    }

    /**
     * Validate create session request parameters
     * @param request Create session request object
     * @throws IllegalArgumentException If request parameters are invalid
     */
    private fun validateCreateSessionRequest(request: CreateSessionRequest) {
        // Validate userId is not blank
        require(request.userId.isNotBlank()) { "userId cannot be blank" }

        // Validate terminal size (if provided) must be positive numbers
        if (request.columns != null) {
            require(request.columns > 0) { "columns must be greater than 0" }
        }

        if (request.rows != null) {
            require(request.rows > 0) { "rows must be greater than 0" }
        }

        // Validate working directory (if provided) cannot be blank
        if (request.workingDirectory != null) {
            require(request.workingDirectory.isNotBlank()) { "workingDirectory cannot be blank" }
        }

        // Validate shell type (if provided) cannot be blank
        if (request.shellType != null) {
            require(request.shellType.isNotBlank()) { "shellType cannot be blank" }
        }
    }
}
