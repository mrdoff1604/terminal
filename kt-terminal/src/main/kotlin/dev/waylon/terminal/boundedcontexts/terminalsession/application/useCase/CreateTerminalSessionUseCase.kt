package dev.waylon.terminal.boundedcontexts.terminalsession.application.useCase

import dev.waylon.terminal.boundedcontexts.terminalsession.application.service.TerminalSessionService
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSession
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSize
import dev.waylon.terminal.boundedcontexts.terminalsession.infrastructure.dto.CreateSessionRequest
import org.slf4j.LoggerFactory

/**
 * 创建终端会话用例
 * 封装创建终端会话的业务逻辑，保持Route层轻量级
 */
class CreateTerminalSessionUseCase(
    private val terminalSessionService: TerminalSessionService
) {
    private val log = LoggerFactory.getLogger(CreateTerminalSessionUseCase::class.java)

    /**
     * 执行创建终端会话操作
     * @param request 创建会话请求对象
     * @return 创建的终端会话
     * @throws IllegalArgumentException 如果请求参数无效
     */
    fun execute(request: CreateSessionRequest): TerminalSession {
        log.debug("Executing CreateTerminalSessionUseCase")
        log.debug("Session creation request: $request")

        // 参数校验
        validateCreateSessionRequest(request)

        // 解析终端尺寸
        val terminalSize = if (request.columns != null && request.rows != null) {
            TerminalSize(request.columns, request.rows)
        } else {
            TerminalSize(80, 24) // 默认尺寸
        }

        // 调用服务层创建会话
        val session = terminalSessionService.createSession(
            userId = request.userId,
            title = request.title,
            shellType = request.shellType,
            workingDirectory = request.workingDirectory,
            size = terminalSize
        )

        log.info(
            "Created new terminal session: {}, shellType: {}, workingDirectory: {}",
            session.id, session.shellType, session.workingDirectory
        )

        return session
    }

    /**
     * 校验创建会话请求参数
     * @param request 创建会话请求对象
     * @throws IllegalArgumentException 如果请求参数无效
     */
    private fun validateCreateSessionRequest(request: CreateSessionRequest) {
        // 校验userId不能为空
        require(request.userId.isNotBlank()) { "userId cannot be blank" }
        
        // 校验终端尺寸（如果提供）必须为正数
        if (request.columns != null) {
            require(request.columns > 0) { "columns must be greater than 0" }
        }
        
        if (request.rows != null) {
            require(request.rows > 0) { "rows must be greater than 0" }
        }
        
        // 校验工作目录（如果提供）不能为空白
        if (request.workingDirectory != null) {
            require(request.workingDirectory.isNotBlank()) { "workingDirectory cannot be blank" }
        }
        
        // 校验shell类型（如果提供）不能为空白
        if (request.shellType != null) {
            require(request.shellType.isNotBlank()) { "shellType cannot be blank" }
        }
    }
}
