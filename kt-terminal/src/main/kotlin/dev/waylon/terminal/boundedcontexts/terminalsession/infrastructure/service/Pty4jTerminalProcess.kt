package dev.waylon.terminal.boundedcontexts.terminalsession.infrastructure.service

import com.pty4j.PtyProcess
import com.pty4j.PtyProcessBuilder
import com.pty4j.WinSize
import dev.waylon.terminal.boundedcontexts.terminalsession.application.process.TerminalProcess
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSize
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.model.TerminalConfig
import java.io.InputStream
import java.io.OutputStream
import java.util.concurrent.TimeUnit

class Pty4jTerminalProcess(
    private val sessionId: String,
    workingDirectory: String,
    shellType: String,
    terminalSize: TerminalSize,
    terminalConfig: TerminalConfig
) : TerminalProcess {
    private val process: PtyProcess
    private var isTerminated = false


    init {
        // Get shell configuration from terminal config
        val shellConfig = terminalConfig.shells[shellType]

        // Use shell configuration from config file, otherwise use default value
        val command = shellConfig?.command?.toTypedArray()

        // Determine working directory
        val actualWorkingDirectory = workingDirectory

        val environment = mutableMapOf<String, String>()

        // Add all system environment variables
        environment.putAll(System.getenv())

        // Override with environment variables from configuration
        if (shellConfig?.environment != null) {
            environment.putAll(shellConfig.environment)
        }

        // Determine terminal size
        val actualSize = terminalSize

        process = PtyProcessBuilder()
            .setCommand(command)
            .setDirectory(actualWorkingDirectory)
            .setEnvironment(environment)
            .setInitialColumns(actualSize.cols)
            .setInitialRows(actualSize.rows)
            .start()

    }

    override fun inputStream(): InputStream {
        return process.inputStream
    }

    override fun outputStream(): OutputStream {
        return process.outputStream
    }

    override fun write(data: String): Boolean {
        if (isTerminated) return false

        return try {
            // Use direct method call instead of reflection for write operation
            // since write is a standard method in OutputStream
            outputStream().write(data.toByteArray())
            outputStream().flush()
            true
        } catch (_: Exception) {
            false
        }
    }

    override fun resize(terminalSize: TerminalSize) {
        if (isTerminated) return

        // Direct call to pty4j 0.13.11 resize method
        process.winSize = WinSize(terminalSize.cols, terminalSize.rows)
    }

    override fun terminate() {
        if (isTerminated) return

        isTerminated = true
        cleanupResources()
    }

    /**
     * Cleanup all resources safely
     */
    private fun cleanupResources() {
        // Mark as terminated to prevent duplicate cleanup
        isTerminated = true

        try {
            // Destroy the process
            if (process.isAlive) {
                process.destroy()
                // Wait for process termination to avoid zombie processes
                process.waitFor(100, TimeUnit.MILLISECONDS)
            }
        } catch (e: Exception) {
            // Ignore process destroy errors, but log them for debugging
        }
    }

    override fun interrupt() {
        if (isTerminated) return

        // Send Ctrl+C signal
        write("\u0003")
    }


    override fun isAlive(): Boolean {
        return !isTerminated && process.isAlive
    }

    /**
     * Use closeable interface to ensure resource release
     */
    fun close() {
        cleanupResources()
    }

}