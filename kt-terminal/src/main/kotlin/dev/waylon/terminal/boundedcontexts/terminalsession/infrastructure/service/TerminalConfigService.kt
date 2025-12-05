package dev.waylon.terminal.boundedcontexts.terminalsession.infrastructure.service

import io.ktor.server.application.Application
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSize
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.model.ShellConfig
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.model.TerminalConfig

/**
 * Terminal Configuration Service
 * Reads terminal configuration from application.conf
 */
class TerminalConfigService(private val application: Application) {

    /**
     * Load terminal configuration from application.conf
     */
    fun loadConfig(): TerminalConfig {
        val config = application.environment.config

        // Read default shell type
        val defaultShellType = config.property("terminal.defaultShellType").getString()

        // Read session timeout in milliseconds
        val sessionTimeoutMs = config.property("terminal.sessionTimeout").getString().toLong()

        // Read default working directory
        val defaultWorkingDirectory = config.property("terminal.defaultWorkingDirectory").getString()

        // Read default terminal size if configured, otherwise use default
        val defaultColumns = config.propertyOrNull("terminal.defaultSize.columns")?.getString()?.toInt() ?: 80
        val defaultRows = config.propertyOrNull("terminal.defaultSize.rows")?.getString()?.toInt() ?: 24
        val defaultTerminalSize = TerminalSize(defaultColumns, defaultRows)

        // Load shells configuration
        val shellsConfig = mutableMapOf<String, ShellConfig>()
        val shellsPath = "terminal.shells"

        // Check if shells configuration exists
        if (config.propertyOrNull(shellsPath) != null) {
            val shells = config.config(shellsPath)
            shells.toMap().keys.forEach { shellName ->
                val shell = shells.config(shellName)

                // Read command
                val command = shell.property("command").getList()

                // Read optional working directory
                val workingDirectory = shell.propertyOrNull("workingDirectory")?.getString()

                // Read optional size
                val size = shell.propertyOrNull("size.columns")?.let {
                    val columns = shell.property("size.columns").getString().toInt()
                    val rows = shell.property("size.rows").getString().toInt()
                    TerminalSize(columns, rows)
                }

                // Read environment variables
                val environment = mutableMapOf<String, String>()
                shell.config("environment").toMap().forEach { entry ->
                    val key = entry.key
                    entry.value?.also { environment[key] = it.toString() }
                }

                shellsConfig[shellName] = ShellConfig(
                    command = command,
                    workingDirectory = workingDirectory,
                    size = size,
                    environment = environment
                )
            }
        }

        return TerminalConfig(
            defaultShellType = defaultShellType,
            sessionTimeoutMs = sessionTimeoutMs,
            defaultWorkingDirectory = defaultWorkingDirectory,
            defaultTerminalSize = defaultTerminalSize,
            shells = shellsConfig
        )
    }

    /**
     * Load terminal configuration asynchronously
     */
    suspend fun loadConfigAsync(): TerminalConfig {
        // 使用Dispatchers.IO处理IO密集型任务
        return withContext(Dispatchers.IO) {
            // 直接调用loadConfig方法，避免代码重复
            loadConfig()
        }
    }
}
