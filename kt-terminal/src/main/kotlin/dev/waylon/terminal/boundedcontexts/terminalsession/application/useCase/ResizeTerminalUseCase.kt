package dev.waylon.terminal.boundedcontexts.terminalsession.application.useCase

import dev.waylon.terminal.boundedcontexts.terminalsession.application.service.TerminalSessionService
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSession
import org.slf4j.LoggerFactory

/**
 * 调整终端大小用例
 * 封装调整终端大小的业务逻辑，保持Route层轻量级
 */
class ResizeTerminalUseCase(
    private val terminalSessionService: TerminalSessionService
) {
    private val log = LoggerFactory.getLogger(ResizeTerminalUseCase::class.java)

    /**
     * 执行调整终端大小操作
     * @param sessionId 会话ID
     * @param columns 列数
     * @param rows 行数
     * @return 调整后的终端会话，如果不存在则返回null
     */
    fun execute(sessionId: String, columns: Int, rows: Int): TerminalSession? {
        log.debug("Executing ResizeTerminalUseCase for sessionId: {}, columns: {}, rows: {}", sessionId, columns, rows)
        
        val session = terminalSessionService.resizeTerminal(sessionId, columns, rows)
        
        if (session != null) {
            log.debug("Resized terminal session {} successfully", sessionId)
        } else {
            log.debug("Failed to resize terminal session {}", sessionId)
        }
        
        return session
    }
}
