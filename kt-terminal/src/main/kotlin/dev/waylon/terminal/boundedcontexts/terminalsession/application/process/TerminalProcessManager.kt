package dev.waylon.terminal.boundedcontexts.terminalsession.application.process

import dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSize

interface TerminalProcessManager {
    fun createProcess(
        sessionId: String,
        workingDirectory: String,
        shellType: String,
        terminalSize: TerminalSize = TerminalSize(80, 24)
    ): TerminalProcess

    fun getProcess(sessionId: String): TerminalProcess?
    fun writeToProcess(sessionId: String, data: String): Boolean
    fun resizeProcess(sessionId: String, terminalSize: TerminalSize): Boolean
    fun terminateProcess(sessionId: String): Boolean
    fun interruptProcess(sessionId: String): Boolean
}