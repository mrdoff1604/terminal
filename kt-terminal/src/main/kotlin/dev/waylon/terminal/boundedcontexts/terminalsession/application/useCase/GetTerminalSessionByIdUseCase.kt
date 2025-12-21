package dev.waylon.terminal.boundedcontexts.terminalsession.application.useCase

import dev.waylon.terminal.boundedcontexts.terminalsession.application.service.TerminalSessionService
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSession
import org.slf4j.LoggerFactory

/**
 * Get terminal session by ID use case
 * Encapsulates the business logic for getting a terminal session by ID, keeping the Route layer lightweight
 */
class GetTerminalSessionByIdUseCase(
    private val terminalSessionService: TerminalSessionService
) : UseCase<String, TerminalSession> {
    private val log = LoggerFactory.getLogger(GetTerminalSessionByIdUseCase::class.java)

    /**
     * Execute the operation to get a terminal session by ID
     * @param sessionId The session ID
     * @return The terminal session
     * @throws TerminalSessionNotFoundException If session not found
     */
    override suspend operator fun invoke(sessionId: String): TerminalSession {
        log.debug("Executing GetTerminalSessionByIdUseCase for sessionId: {}", sessionId)
        return terminalSessionService.getSessionById(sessionId).also {
            if (it != null) {
                log.debug("Found session: {}, status: {}", sessionId, it.status)
            } else {
                log.debug("Session not found: {}", sessionId)
            }
        }
    }
}
