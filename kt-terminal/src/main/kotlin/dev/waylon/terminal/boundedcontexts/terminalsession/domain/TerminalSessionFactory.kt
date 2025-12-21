package dev.waylon.terminal.boundedcontexts.terminalsession.domain

import dev.waylon.terminal.boundedcontexts.terminalsession.domain.model.TerminalConfig
import java.util.UUID

/**
 * Terminal Session Factory
 * Responsible for creating TerminalSession objects, handling parameter priority:
 * 1. Request parameters
 * 2. Configured shell parameters
 * 3. Default shell parameters
 */
class TerminalSessionFactory(
    private val terminalConfig: TerminalConfig
) {
    private val defaultShellType = terminalConfig.defaultShellType
    private val defaultWorkingDirectory = terminalConfig.defaultWorkingDirectory
    private val defaultTerminalSize = terminalConfig.defaultTerminalSize
    private val sessionTimeoutMs = terminalConfig.sessionTimeoutMs
    
    /**
     * Create Terminal Session
     * @param userId User ID
     * @param title Session title
     * @param workingDirectory Requested working directory
     * @param shellType Requested shell type
     * @param terminalSize Requested terminal size
     * @return Created terminal session
     */
    fun createSession(
        userId: String,
        title: String?,
        workingDirectory: String?,
        shellType: String?,
        terminalSize: TerminalSize?
    ): TerminalSession {
        val now = System.currentTimeMillis()
        
        // 1. Determine actual shell type: request parameter > default value
        val actualShellType = shellType ?: defaultShellType
        
        // 2. Get shell configuration
        val shellConfig = terminalConfig.shells[actualShellType]
        
        // 3. Determine actual working directory: request parameter > shell config > default value
        val actualWorkingDirectory = when {
            workingDirectory != null && workingDirectory.isNotBlank() -> workingDirectory
            shellConfig?.workingDirectory != null -> shellConfig.workingDirectory
            else -> defaultWorkingDirectory
        }
        
        // 4. Determine actual terminal size: request parameter > shell config > default value
        val actualTerminalSize = when {
            terminalSize != null -> terminalSize
            shellConfig?.size != null -> shellConfig.size
            else -> defaultTerminalSize
        }
        
        // 5. Create terminal session
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
