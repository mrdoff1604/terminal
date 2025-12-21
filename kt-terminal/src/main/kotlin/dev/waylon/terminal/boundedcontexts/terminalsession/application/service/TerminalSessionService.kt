package dev.waylon.terminal.boundedcontexts.terminalsession.application.service

import dev.waylon.terminal.boundedcontexts.terminalsession.application.process.TerminalProcessManager
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSession
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSessionFactory
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSessionRepository
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSessionStatus
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSize
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.model.TerminalConfig
import org.slf4j.LoggerFactory

/**
 * Terminal Session Service
 * Responsible for coordinating the lifecycle management of terminal sessions
 * Follows DDD best practices: Service layer only coordinates, no business logic
 * Follows DIP: Depends on abstractions, not concrete implementations
 */
class TerminalSessionService(
    private val terminalConfig: TerminalConfig,
    private val terminalSessionRepository: TerminalSessionRepository,
    private val terminalProcessManager: TerminalProcessManager,
    private val terminalSessionFactory: TerminalSessionFactory
) {
    private val log = LoggerFactory.getLogger(TerminalSessionService::class.java)

    /**
     * Create terminal session
     * @param userId User ID
     * @param title Session title
     * @param workingDirectory Working directory
     * @param shellType Shell type
     * @param size Terminal size
     * @return Created terminal session
     */
    fun createSession(
        userId: String,
        title: String?,
        shellType: String?,
        workingDirectory: String?,
        size: TerminalSize?
    ): TerminalSession {
        // Use factory to create session, factory handles parameter priority
        val session = terminalSessionFactory.createSession(
            userId = userId,
            title = title,
            workingDirectory = workingDirectory,
            shellType = shellType,
            terminalSize = size
        )
        
        log.debug("TerminalSession created. {}", session)
        terminalSessionRepository.save(session)

        return session
    }

    /**
     * Get terminal session by ID
     * @param id Session ID
     * @return Terminal session, or null if not found
     */
    fun getSessionById(id: String): TerminalSession? {
        return terminalSessionRepository.getById(id)?.also {
            updateSessionActivity(it)
        }
    }

    /**
     * Get terminal sessions by user ID
     * @param userId User ID
     * @return List of terminal sessions
     */
    fun getSessionsByUserId(userId: String): List<TerminalSession> {
        return terminalSessionRepository.getByUserId(userId)
    }

    /**
     * Get all terminal sessions
     * @return List of all terminal sessions
     */
    fun getAllSessions(): List<TerminalSession> {
        return terminalSessionRepository.getAll()
    }

    /**
     * Resize terminal
     * @param id Session ID
     * @param columns Number of columns
     * @param rows Number of rows
     * @return Resized terminal session, or null if not found
     */
    fun resizeTerminal(id: String, columns: Int, rows: Int): TerminalSession? {
        val terminalSize = TerminalSize(columns, rows)
        
        // 1. First update PTY process size
        val resizeSuccess = terminalProcessManager.resizeProcess(id, terminalSize)
        log.debug("Resize PTY process result for session {}: {}", id, resizeSuccess)
        
        // 2. Then update session object in storage
        return terminalSessionRepository.getById(id)?.also {
            it.resize(columns, rows)
            terminalSessionRepository.update(it)
        }
    }

    /**
     * Terminate terminal session
     * @param id Session ID
     * @param reason Termination reason
     * @return Terminated terminal session, or null if not found
     */
    fun terminateSession(id: String, reason: String? = null): TerminalSession? {
        return terminalSessionRepository.getById(id)?.also {
            // Use domain model's terminate method
            it.terminate()

            // Remove from storage
            terminalSessionRepository.deleteById(id)
        }
    }

    /**
     * Update terminal session status
     * @param id Session ID
     * @param status New status
     * @return Updated terminal session, or null if not found
     */
    fun updateSessionStatus(id: String, status: TerminalSessionStatus): TerminalSession? {
        return terminalSessionRepository.getById(id)?.also {
            // Use domain model's updateStatus method
            it.updateStatus(status)
            terminalSessionRepository.update(it)
        }
    }

    /**
     * Delete terminal session
     * @param id Session ID
     * @return Whether deletion was successful
     */
    fun deleteSession(id: String): Boolean {
        val session = terminalSessionRepository.deleteById(id)
        return session != null
    }

    /**
     * Update session activity time
     * @param session Terminal session
     */
    private fun updateSessionActivity(session: TerminalSession) {
        val now = System.currentTimeMillis()

        // Use domain model methods to update activity time and expiry time
        session.updateActivity(now)
        session.updateExpiryTime(terminalConfig.sessionTimeoutMs, now)

        // Update session in storage
        terminalSessionRepository.update(session)
    }
}
