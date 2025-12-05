package dev.waylon.terminal.boundedcontexts.terminalsession.application.useCase

import dev.waylon.terminal.boundedcontexts.terminalsession.application.service.TerminalSessionService
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSession
import org.slf4j.LoggerFactory

/**
 * 获取所有终端会话用例
 * 封装获取所有终端会话的业务逻辑，保持Route层轻量级
 */
class GetAllTerminalSessionsUseCase(
    private val terminalSessionService: TerminalSessionService
) {
    private val log = LoggerFactory.getLogger(GetAllTerminalSessionsUseCase::class.java)

    /**
     * 执行获取所有终端会话操作
     * @return 所有终端会话列表
     */
    fun execute(): List<TerminalSession> {
        log.debug("Executing GetAllTerminalSessionsUseCase")
        
        val sessions = terminalSessionService.getAllSessions()
        
        log.debug("Found {} terminal sessions", sessions.size)
        
        return sessions
    }
}
