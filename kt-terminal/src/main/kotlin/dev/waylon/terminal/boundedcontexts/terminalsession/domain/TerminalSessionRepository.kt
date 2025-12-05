package dev.waylon.terminal.boundedcontexts.terminalsession.domain

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
     * Saves a terminal session to the repository.
     * 
     * @param session The terminal session to save
     */
    fun save(session: TerminalSession)

    /**
     * Retrieves a terminal session by its ID.
     * 
     * @param id The unique identifier of the session to retrieve
     * @return The terminal session if found, null otherwise
     */
    fun getById(id: String): TerminalSession?

    /**
     * Retrieves all terminal sessions from the repository.
     * 
     * @return A list of all terminal sessions
     */
    fun getAll(): List<TerminalSession>

    /**
     * Retrieves all terminal sessions for a specific user.
     * 
     * @param userId The unique identifier of the user
     * @return A list of terminal sessions associated with the user
     */
    fun getByUserId(userId: String): List<TerminalSession>

    /**
     * Updates an existing terminal session in the repository.
     * 
     * @param session The terminal session with updated information
     */
    fun update(session: TerminalSession)

    /**
     * Deletes a terminal session by its ID.
     * 
     * @param id The unique identifier of the session to delete
     * @return The deleted terminal session if found, null otherwise
     */
    fun deleteById(id: String): TerminalSession?

    /**
     * Deletes all terminal sessions from the repository.
     */
    fun deleteAll()
}
