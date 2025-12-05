package dev.waylon.terminal.boundedcontexts.terminalsession.infrastructure.repository

import dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSession
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSessionRepository

/**
 * In-memory implementation of TerminalSessionRepository
 * Infrastructure layer implementation, separated from domain interface
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
