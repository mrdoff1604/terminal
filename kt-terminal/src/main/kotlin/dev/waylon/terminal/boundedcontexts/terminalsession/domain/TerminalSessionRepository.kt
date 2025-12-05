package dev.waylon.terminal.boundedcontexts.terminalsession.domain

/**
 * Terminal Session Repository Interface
 * Defines the contract for session storage operations
 * This allows us to support different storage implementations (memory, Redis, etc.)
 * 
 * Following DIP: Abstractions should not depend on details. Details should depend on abstractions.
 */
interface TerminalSessionRepository {
    /**
     * Save a session
     */
    fun save(session: TerminalSession)

    /**
     * Get a session by ID
     */
    fun getById(id: String): TerminalSession?

    /**
     * Get all sessions
     */
    fun getAll(): List<TerminalSession>

    /**
     * Get sessions by user ID
     */
    fun getByUserId(userId: String): List<TerminalSession>

    /**
     * Update a session
     */
    fun update(session: TerminalSession)

    /**
     * Delete a session by ID
     */
    fun deleteById(id: String): TerminalSession?

    /**
     * Delete all sessions
     */
    fun deleteAll()
}
