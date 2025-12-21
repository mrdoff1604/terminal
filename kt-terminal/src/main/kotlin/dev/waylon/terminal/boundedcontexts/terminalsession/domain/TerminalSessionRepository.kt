package dev.waylon.terminal.boundedcontexts.terminalsession.domain

import kotlinx.coroutines.flow.Flow

/**
 * Terminal Session Repository Interface
 * 
 * Defines the contract for session storage operations. This allows us to support 
 * different storage implementations (memory, Redis, database, etc.) without 
 * changing the domain or application layer code.
 * 
 * Following DIP: Abstractions should not depend on details. Details should depend on abstractions.
 * 
 * @see InMemoryTerminalSessionRepository
 */
interface TerminalSessionRepository {
    /**
     * Saves a terminal session to the repository asynchronously.
     * 
     * @param session The terminal session to save
     */
    suspend fun save(session: TerminalSession)

    /**
     * Retrieves a terminal session by its ID asynchronously.
     * 
     * @param id The unique identifier of the session to retrieve
     * @return The terminal session if found, null otherwise
     */
    suspend fun getById(id: String): TerminalSession?

    /**
     * Retrieves all terminal sessions from the repository asynchronously.
     * 
     * @return A Flow of all terminal sessions
     */
    fun getAll(): Flow<TerminalSession>

    /**
     * Retrieves all terminal sessions for a specific user asynchronously.
     * 
     * @param userId The unique identifier of the user
     * @return A Flow of terminal sessions associated with the user
     */
    fun getByUserId(userId: String): Flow<TerminalSession>

    /**
     * Updates an existing terminal session in the repository asynchronously.
     * 
     * @param session The terminal session with updated information
     */
    suspend fun update(session: TerminalSession)

    /**
     * Deletes a terminal session by its ID asynchronously.
     * 
     * @param id The unique identifier of the session to delete
     * @return The deleted terminal session if found, null otherwise
     */
    suspend fun deleteById(id: String): TerminalSession?

    /**
     * Deletes all terminal sessions from the repository asynchronously.
     */
    suspend fun deleteAll()
}
