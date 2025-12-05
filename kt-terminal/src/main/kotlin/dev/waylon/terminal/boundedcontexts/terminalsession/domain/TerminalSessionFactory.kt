package dev.waylon.terminal.boundedcontexts.terminalsession.domain

import dev.waylon.terminal.boundedcontexts.terminalsession.domain.model.TerminalConfig
import java.util.UUID

/**
 * 终端会话工厂
 * 负责创建TerminalSession对象，处理参数优先级：
 * 1. 请求参数
 * 2. 配置的shell参数
 * 3. 默认的shell参数
 */
class TerminalSessionFactory(
    private val terminalConfig: TerminalConfig
) {
    private val defaultShellType = terminalConfig.defaultShellType
    private val defaultWorkingDirectory = terminalConfig.defaultWorkingDirectory
    private val defaultTerminalSize = terminalConfig.defaultTerminalSize
    private val sessionTimeoutMs = terminalConfig.sessionTimeoutMs
    
    /**
     * 创建终端会话
     * @param userId 用户ID
     * @param title 会话标题
     * @param workingDirectory 请求的工作目录
     * @param shellType 请求的shell类型
     * @param terminalSize 请求的终端尺寸
     * @return 创建的终端会话
     */
    fun createSession(
        userId: String,
        title: String?,
        workingDirectory: String?,
        shellType: String?,
        terminalSize: TerminalSize?
    ): TerminalSession {
        val now = System.currentTimeMillis()
        
        // 1. 确定实际使用的shell类型：请求参数 > 默认值
        val actualShellType = shellType ?: defaultShellType
        
        // 2. 获取shell配置
        val shellConfig = terminalConfig.shells[actualShellType]
        
        // 3. 确定实际工作目录：请求参数 > shell配置 > 默认值
        val actualWorkingDirectory = when {
            workingDirectory != null && workingDirectory.isNotBlank() -> workingDirectory
            shellConfig?.workingDirectory != null -> shellConfig.workingDirectory
            else -> defaultWorkingDirectory
        }
        
        // 4. 确定实际终端尺寸：请求参数 > shell配置 > 默认值
        val actualTerminalSize = when {
            terminalSize != null -> terminalSize
            shellConfig?.size != null -> shellConfig.size
            else -> defaultTerminalSize
        }
        
        // 5. 创建终端会话
        return TerminalSession(
            id = UUID.randomUUID().toString(),
            userId = userId,
            title = title,
            workingDirectory = actualWorkingDirectory,
            shellType = actualShellType,
            status = TerminalSessionStatus.ACTIVE,
            terminalSize = actualTerminalSize,
            createdAt = now,
            updatedAt = now,
            lastActiveTime = now,
            expiredAt = now + sessionTimeoutMs
        )
    }
}
