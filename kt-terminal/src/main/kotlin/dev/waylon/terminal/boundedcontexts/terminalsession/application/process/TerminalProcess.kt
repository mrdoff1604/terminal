package dev.waylon.terminal.boundedcontexts.terminalsession.application.process

import dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSize
import java.io.InputStream
import java.io.OutputStream

interface TerminalProcess {
    fun inputStream(): InputStream
    fun outputStream(): OutputStream
    fun write(data: String): Boolean
    fun resize(terminalSize: TerminalSize)
    fun terminate()
    fun interrupt()
    fun isAlive(): Boolean
}