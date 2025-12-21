package dev.waylon.terminal.boundedcontexts.terminalsession.application.service

import dev.waylon.terminal.boundedcontexts.terminalsession.application.process.TerminalProcessManager
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSession
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSessionFactory
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSessionRepository
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSessionStatus
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSize
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.exception.TerminalSessionNotFoundException
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.model.TerminalConfig
import kotlin.time.Clock
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.filter
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
     * Create terminal session asynchronously
     * @param userId User ID
     * @param title Session title
     * @param workingDirectory Working directory
     * @param shellType Shell type
     * @param size Terminal size
     * @return Created terminal session
     */
    suspend fun createSession(
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
     * Get terminal session by ID asynchronously
     * @param id Session ID
     * @return Terminal session
     * @throws TerminalSessionNotFoundException If session not found
     */
    suspend fun getSessionById(id: String): TerminalSession {
        return terminalSessionRepository.getById(id)?.let {
            updateSessionActivity(it)
            it
        } ?: throw TerminalSessionNotFoundException(id)
    }

    /**
     * Get terminal sessions by user ID asynchronously
     * @param userId User ID
     * @return Flow of active (not expired) terminal sessions
     */
    fun getSessionsByUserId(userId: String): Flow<TerminalSession> {
        return terminalSessionRepository.getByUserId(userId)
            .filter { !it.isExpired() }
    }

    /**
     * Get all terminal sessions asynchronously
     * @return Flow of active (not expired) terminal sessions
     */
    fun getAllSessions(): Flow<TerminalSession> {
        return terminalSessionRepository.getAll()
            .filter { !it.isExpired() }
    }

    /**
     * Resize terminal asynchronously
     * @param id Session ID
     * @param columns Number of columns
     * @param rows Number of rows
     * @return Resized terminal session
     * @throws TerminalSessionNotFoundException If session not found
     */
    suspend fun resizeTerminal(id: String, columns: Int, rows: Int): TerminalSession {
        val terminalSize = TerminalSize(columns, rows)

        // 1. First update PTY process size
        terminalProcessManager.resizeProcess(id, terminalSize).also {
            log.debug("Resize PTY process result for session {}: {}", id, it)
        }

        // 2. Then update session object in storage
        return getSessionById(id).apply {
            resize(columns, rows)
            terminalSessionRepository.update(this)
        }
    }

    /**
     * Terminate terminal session asynchronously
     * @param id Session ID
     * @param reason Termination reason
     * @return Terminated terminal session
     * @throws TerminalSessionNotFoundException If session not found
     */
    suspend fun terminateSession(id: String, reason: String? = null): TerminalSession {
        return getSessionById(id).apply {
            // Use domain model's terminate method
            terminate()
        }.also {
            // Remove from storage
            terminalSessionRepository.deleteById(id)
        }
    }

    /**
     * Update terminal session status asynchronously
     * @param id Session ID
     * @param status New status
     * @return Updated terminal session
     * @throws TerminalSessionNotFoundException If session not found
     */
    suspend fun updateSessionStatus(id: String, status: TerminalSessionStatus): TerminalSession {
        return getSessionById(id).apply {
            // Use domain model's updateStatus method
            updateStatus(status)
            terminalSessionRepository.update(this)
        }
    }

    /**
     * Delete terminal session asynchronously
     * @param id Session ID
     * @return Whether deletion was successful
     */
    suspend fun deleteSession(id: String): Boolean {
        val session = terminalSessionRepository.deleteById(id)
        return session != null
    }

    /**
     * Update session activity time asynchronously
     * @param session Terminal session
     */
    private suspend fun updateSessionActivity(session: TerminalSession) {
        val now = Clock.System.now()

        // Use domain model methods to update activity time and expiry time
        session.updateActivity(now)
        session.updateExpiryTime(terminalConfig.sessionTimeoutMs, now)

        // Update session in storage
        terminalSessionRepository.update(session)
    }
}
