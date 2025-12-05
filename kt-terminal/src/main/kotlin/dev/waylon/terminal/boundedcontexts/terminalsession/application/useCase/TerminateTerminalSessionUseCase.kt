package dev.waylon.terminal.boundedcontexts.terminalsession.application.useCase

import dev.waylon.terminal.boundedcontexts.terminalsession.application.service.TerminalSessionService
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSession
import org.slf4j.LoggerFactory

/**
 * 终止终端会话用例
 * 封装终止终端会话的业务逻辑，保持Route层轻量级
 */
class TerminateTerminalSessionUseCase(
    private val terminalSessionService: TerminalSessionService
) {
    private val log = LoggerFactory.getLogger(TerminateTerminalSessionUseCase::class.java)

    /**
     * 执行终止终端会话操作
     * @param sessionId 会话ID
     * @return 终止的终端会话，如果不存在则返回null
     */
    fun execute(sessionId: String): TerminalSession? {
        log.debug("Executing TerminateTerminalSessionUseCase for sessionId: {}", sessionId)
        
        val session = terminalSessionService.terminateSession(sessionId)
        
        if (session != null) {
            log.info("Terminated terminal session: {}", sessionId)
        } else {
            log.debug("Failed to terminate terminal session: {}", sessionId)
        }
        
        return session
    }
}
