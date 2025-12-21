package dev.waylon.terminal.boundedcontexts.terminalsession.application.useCase

import dev.waylon.terminal.boundedcontexts.terminalsession.application.service.TerminalSessionService
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSession
import kotlinx.coroutines.flow.Flow
import org.slf4j.LoggerFactory

/**
 * Get all terminal sessions use case
 * Encapsulates the business logic for getting all terminal sessions, keeping the Route layer lightweight
 */
class GetAllTerminalSessionsUseCase(
    private val terminalSessionService: TerminalSessionService
) {
    private val log = LoggerFactory.getLogger(GetAllTerminalSessionsUseCase::class.java)

    /**
     * Execute the operation to get all terminal sessions
     * @return Flow of all terminal sessions
     */
    fun execute(): Flow<TerminalSession> {
        log.debug("Executing GetAllTerminalSessionsUseCase")

        val sessions = terminalSessionService.getAllSessions()

        log.debug("Found terminal sessions")

        return sessions
    }
}
