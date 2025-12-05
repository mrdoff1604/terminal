package dev.waylon.terminal.boundedcontexts.terminalsession.domain.model

import dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSize

/**
 * Shell Configuration Model
 * Represents the configuration for a specific shell type
 */
data class ShellConfig(
    val command: List<String>,
    val workingDirectory: String?,
    val size: TerminalSize?,
    val environment: Map<String, String> = emptyMap()
)

/**
 * Terminal Configuration Model
 * Represents the configuration for terminal sessions
 */
data class TerminalConfig(
    val defaultShellType: String,
    val sessionTimeoutMs: Long,
    val defaultWorkingDirectory: String,
    val defaultTerminalSize: TerminalSize = TerminalSize(80, 24),
    val shells: Map<String, ShellConfig> = emptyMap()
)

