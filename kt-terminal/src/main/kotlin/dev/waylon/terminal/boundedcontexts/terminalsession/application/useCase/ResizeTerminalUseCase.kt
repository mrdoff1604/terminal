package dev.waylon.terminal.boundedcontexts.terminalsession.application.useCase

import dev.waylon.terminal.boundedcontexts.terminalsession.application.service.TerminalSessionService
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSession
import dev.waylon.terminal.boundedcontexts.terminalsession.infrastructure.dto.ResizeTerminalRequest
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
     * @param request 调整终端大小请求对象
     * @return 调整后的终端会话，如果不存在则返回null
     * @throws IllegalArgumentException 如果请求参数无效
     */
    fun execute(sessionId: String, request: ResizeTerminalRequest): TerminalSession? {
        log.debug("Executing ResizeTerminalUseCase for sessionId: {}, request: {}", sessionId, request)
        
        // 参数校验
        validateResizeTerminalRequest(request)
        
        val session = terminalSessionService.resizeTerminal(sessionId, request.columns, request.rows)
        
        if (session != null) {
            log.debug("Resized terminal session {} successfully", sessionId)
        } else {
            log.debug("Failed to resize terminal session {}", sessionId)
        }
        
        return session
    }

    /**
     * 校验调整终端大小请求参数
     * @param request 调整终端大小请求对象
     * @throws IllegalArgumentException 如果请求参数无效
     */
    private fun validateResizeTerminalRequest(request: ResizeTerminalRequest) {
        // 校验列数必须为正数
        require(request.columns > 0) { "columns must be greater than 0" }
        
        // 校验行数必须为正数
        require(request.rows > 0) { "rows must be greater than 0" }
        
        // 可以添加更多的业务规则校验，比如最大尺寸限制
        // require(request.columns <= 1000) { "columns must be less than or equal to 1000" }
        // require(request.rows <= 1000) { "rows must be less than or equal to 1000" }
    }
}
