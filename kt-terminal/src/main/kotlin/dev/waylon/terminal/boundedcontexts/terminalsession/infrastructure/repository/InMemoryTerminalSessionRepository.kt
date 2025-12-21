package dev.waylon.terminal.boundedcontexts.terminalsession.infrastructure.repository

import dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSession
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSessionRepository
import kotlinx.coroutines.flow.asFlow

/**
 * In-memory implementation of TerminalSessionRepository
 * Infrastructure layer implementation, separated from domain interface
 */
class InMemoryTerminalSessionRepository : TerminalSessionRepository {
    private val sessions = mutableMapOf<String, TerminalSession>()

    override suspend fun save(session: TerminalSession) {
        sessions[session.id] = session
    }

    override suspend fun getById(id: String): TerminalSession? {
        return sessions[id]
    }

    override fun getAll(): kotlinx.coroutines.flow.Flow<TerminalSession> {
        return sessions.values.asFlow()
    }

    override fun getByUserId(userId: String): kotlinx.coroutines.flow.Flow<TerminalSession> {
        return sessions.values.filter { it.userId == userId }.asFlow()
    }

    override suspend fun update(session: TerminalSession) {
        sessions[session.id] = session
    }

    override suspend fun deleteById(id: String): TerminalSession? {
        return sessions.remove(id)
    }

    override suspend fun deleteAll() {
        sessions.clear()
    }
}
