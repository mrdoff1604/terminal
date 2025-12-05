package dev.waylon.terminal.boundedcontexts.terminalsession.application.useCase

import dev.waylon.terminal.boundedcontexts.terminalsession.application.service.TerminalSessionService
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSession
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSize
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
     * @param userId 用户ID
     * @param title 会话标题
     * @param workingDirectory 工作目录
     * @param shellType Shell类型
     * @param columns 终端列数
     * @param rows 终端行数
     * @return 创建的终端会话
     */
    fun execute(
        userId: String,
        title: String?,
        workingDirectory: String?,
        shellType: String?,
        columns: String?,
        rows: String?
    ): TerminalSession {
        log.debug("Executing CreateTerminalSessionUseCase")
        log.debug(
            "Session creation parameters: userId={}, title={}, workingDirectory={}, shellType={}, columns={}, rows={}",
            userId, title, workingDirectory, shellType, columns, rows
        )

        // 解析终端尺寸
        val terminalSize = if (columns != null && rows != null) {
            TerminalSize(columns.toInt(), rows.toInt())
        } else {
            TerminalSize(80, 24) // 默认尺寸
        }

        // 调用服务层创建会话
        val session = terminalSessionService.createSession(
            userId = userId,
            title = title,
            shellType = shellType,
            workingDirectory = workingDirectory,
            size = terminalSize
        )

        log.info(
            "Created new terminal session: {}, shellType: {}, workingDirectory: {}",
            session.id, session.shellType, session.workingDirectory
        )

        return session
    }
}
