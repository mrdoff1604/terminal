package dev.waylon.terminal.boundedcontexts.terminalsession.application.process

import dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSize
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.model.TerminalConfig
import dev.waylon.terminal.boundedcontexts.terminalsession.infrastructure.service.Pty4jTerminalProcess

/**
 * 终端进程工厂
 * 负责创建TerminalProcess对象，处理参数优先级：
 * 1. 请求参数
 * 2. 配置的shell参数
 * 3. 默认的shell参数
 */
class TerminalProcessFactory(
    private val terminalConfig: TerminalConfig
) {
    /**
     * 创建终端进程
     * @param sessionId 会话ID
     * @param requestedWorkingDirectory 请求的工作目录
     * @param requestedShellType 请求的shell类型
     * @param requestedTerminalSize 请求的终端尺寸
     * @return 创建的终端进程
     */
    fun createProcess(
        sessionId: String,
        requestedWorkingDirectory: String,
        requestedShellType: String,
        requestedTerminalSize: TerminalSize
    ): TerminalProcess {
        // 1. 确定实际使用的shell类型：请求参数 > 默认值
        val actualShellType = requestedShellType.ifBlank { terminalConfig.defaultShellType }
        
        // 2. 获取shell配置
        val shellConfig = terminalConfig.shells[actualShellType]
        
        // 3. 确定实际工作目录：请求参数 > shell配置 > 默认值
        val actualWorkingDirectory = when {
            requestedWorkingDirectory.isNotBlank() -> requestedWorkingDirectory
            shellConfig?.workingDirectory?.isNotBlank() == true -> shellConfig.workingDirectory
            else -> terminalConfig.defaultWorkingDirectory
        }
        
        // 4. 确定实际终端尺寸：请求参数 > shell配置 > 默认值
        val actualTerminalSize = when {
            requestedTerminalSize != terminalConfig.defaultTerminalSize -> requestedTerminalSize
            shellConfig?.size != null -> shellConfig.size
            else -> terminalConfig.defaultTerminalSize
        }
        
        // 5. 创建终端进程
        return Pty4jTerminalProcess(
            sessionId = sessionId,
            workingDirectory = actualWorkingDirectory,
            shellType = actualShellType,
            terminalSize = actualTerminalSize,
            terminalConfig = terminalConfig
        )
    }
}
