package dev.waylon.terminal.boundedcontexts.terminalsession.application.useCase

import dev.waylon.terminal.boundedcontexts.terminalsession.application.service.TerminalSessionService
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSession
import org.slf4j.LoggerFactory

/**
 * 根据ID获取终端会话用例
 * 封装根据ID获取终端会话的业务逻辑，保持Route层轻量级
 */
class GetTerminalSessionByIdUseCase(
    private val terminalSessionService: TerminalSessionService
) {
    private val log = LoggerFactory.getLogger(GetTerminalSessionByIdUseCase::class.java)

    /**
     * 执行根据ID获取终端会话操作
     * @param sessionId 会话ID
     * @return 终端会话，如果不存在则返回null
     */
    fun execute(sessionId: String): TerminalSession? {
        log.debug("Executing GetTerminalSessionByIdUseCase for sessionId: {}", sessionId)
        
        val session = terminalSessionService.getSessionById(sessionId)
        
        if (session != null) {
            log.debug("Found session: {}, status: {}", sessionId, session.status)
        } else {
            log.debug("Session not found: {}", sessionId)
        }
        
        return session
    }
}
