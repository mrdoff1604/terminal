package dev.waylon.terminal.boundedcontexts.terminalsession.infrastructure.service

import dev.waylon.terminal.boundedcontexts.terminalsession.application.process.TerminalProcess
import dev.waylon.terminal.boundedcontexts.terminalsession.application.process.TerminalProcessFactory
import dev.waylon.terminal.boundedcontexts.terminalsession.application.process.TerminalProcessManager
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSize
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.model.TerminalConfig
import java.util.concurrent.ConcurrentHashMap
import org.slf4j.LoggerFactory

private val log = LoggerFactory.getLogger(Pty4jTerminalProcessManager::class.java)

// Concrete implementation using pty4j - should be in infrastructure layer
class Pty4jTerminalProcessManager(
    private val terminalConfig: TerminalConfig,
    private val terminalProcessFactory: TerminalProcessFactory
) : TerminalProcessManager {
    private val processes = ConcurrentHashMap<String, TerminalProcess>()

    override fun createProcess(
        sessionId: String,
        workingDirectory: String,
        shellType: String,
        terminalSize: TerminalSize
    ): TerminalProcess {
        val process = terminalProcessFactory.createProcess(sessionId, workingDirectory, shellType, terminalSize)
        processes[sessionId] = process
        return process
    }

    override fun getProcess(sessionId: String): TerminalProcess? {
        return processes[sessionId]
    }

    override fun writeToProcess(sessionId: String, data: String): Boolean {
        val process = processes[sessionId] ?: return false
        return process.write(data)
    }

    override fun resizeProcess(sessionId: String, terminalSize: TerminalSize): Boolean {
        val process = processes[sessionId] ?: return false
        process.resize(terminalSize)
        return true
    }

    override fun terminateProcess(sessionId: String): Boolean {
        val process = processes.remove(sessionId) ?: return false
        process.terminate()
        return true
    }

    override fun interruptProcess(sessionId: String): Boolean {
        val process = processes[sessionId] ?: return false
        process.interrupt()
        return true
    }
}

