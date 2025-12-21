package dev.waylon.terminal.boundedcontexts.terminalsession.infrastructure.repository

import dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSession
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSessionRepository
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.asFlow
import kotlinx.coroutines.flow.filter

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

    override fun getAll(): Flow<TerminalSession> {
        return sessions.values.asFlow()
    }

    override fun getByUserId(userId: String): Flow<TerminalSession> {
        return sessions.values.asFlow()
            .filter { it.userId == userId }
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
