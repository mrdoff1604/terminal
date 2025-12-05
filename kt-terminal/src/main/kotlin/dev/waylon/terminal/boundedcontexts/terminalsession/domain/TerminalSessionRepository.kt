package dev.waylon.terminal.boundedcontexts.terminalsession.domain

/**
 * Session Storage Interface
 * Defines the contract for session storage operations
 * This allows us to support different storage implementations (memory, Redis, etc.)
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

/**
 * In-memory implementation of SessionStorage
 */
class InMemoryTerminalSessionRepository : TerminalSessionRepository {
    private val sessions = mutableMapOf<String, TerminalSession>()

    override fun save(session: TerminalSession) {
        sessions[session.id] = session
    }

    override fun getById(id: String): TerminalSession? {
        return sessions[id]
    }

    override fun getAll(): List<TerminalSession> {
        return sessions.values.toList()
    }

    override fun getByUserId(userId: String): List<TerminalSession> {
        return sessions.values.filter { it.userId == userId }
    }

    override fun update(session: TerminalSession) {
        sessions[session.id] = session
    }

    override fun deleteById(id: String): TerminalSession? {
        return sessions.remove(id)
    }

    override fun deleteAll() {
        sessions.clear()
    }
}
