package dev.waylon.terminal.boundedcontexts.terminalsession.application.process

import dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSize
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.model.TerminalConfig
import dev.waylon.terminal.boundedcontexts.terminalsession.infrastructure.service.Pty4jTerminalProcess

/**
 * Terminal Process Factory
 * Responsible for creating TerminalProcess objects, handling parameter priority:
 * 1. Request parameters
 * 2. Configured shell parameters
 * 3. Default shell parameters
 */
class TerminalProcessFactory(
    private val terminalConfig: TerminalConfig
) {
    /**
     * Create Terminal Process
     * @param sessionId Session ID
     * @param requestedWorkingDirectory Requested working directory
     * @param requestedShellType Requested shell type
     * @param requestedTerminalSize Requested terminal size
     * @return Created terminal process
     */
    fun createProcess(
        sessionId: String,
        requestedWorkingDirectory: String,
        requestedShellType: String,
        requestedTerminalSize: TerminalSize
    ): TerminalProcess {
        // 1. Determine actual shell type: request parameter > default value
        val actualShellType = requestedShellType.ifBlank { terminalConfig.defaultShellType }

        // 2. Get shell configuration
        val shellConfig = terminalConfig.shells[actualShellType]

        // 3. Determine actual working directory: request parameter > shell config > default value
        val actualWorkingDirectory = when {
            requestedWorkingDirectory.isNotBlank() -> requestedWorkingDirectory
            shellConfig?.workingDirectory?.isNotBlank() == true -> shellConfig.workingDirectory
            else -> terminalConfig.defaultWorkingDirectory
        }

        // 4. Determine actual terminal size: request parameter > shell config > default value
        val actualTerminalSize = when {
            requestedTerminalSize != terminalConfig.defaultTerminalSize -> requestedTerminalSize
            shellConfig?.size != null -> shellConfig.size
            else -> terminalConfig.defaultTerminalSize
        }

        // 5. Create terminal process
        return Pty4jTerminalProcess(
            sessionId = sessionId,
            workingDirectory = actualWorkingDirectory,
            shellType = actualShellType,
            terminalSize = actualTerminalSize,
            terminalConfig = terminalConfig
        )
    }
}
