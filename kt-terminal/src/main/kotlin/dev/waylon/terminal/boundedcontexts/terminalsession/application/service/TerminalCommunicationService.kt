package dev.waylon.terminal.boundedcontexts.terminalsession.application.service

import dev.waylon.terminal.boundedcontexts.terminalsession.application.TerminalCommunicationProtocol
import dev.waylon.terminal.boundedcontexts.terminalsession.application.process.TerminalProcessManager
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.exception.TerminalSessionNotFoundException
import kotlinx.coroutines.CompletableDeferred
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.Job
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.cancel
import kotlinx.coroutines.channels.Channel
import kotlinx.coroutines.isActive
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import org.slf4j.LoggerFactory

class TerminalCommunicationService(
    private val terminalSessionService: TerminalSessionService,
    private val terminalProcessManager: TerminalProcessManager,
) {

    private val log = LoggerFactory.getLogger(TerminalCommunicationService::class.java)

    suspend fun handleCommunication(sessionId: String, protocol: TerminalCommunicationProtocol) {
        log.info("Handling communication for session: $sessionId")

        val terminalSession = try {
            terminalSessionService.getSessionById(sessionId)
        } catch (e: TerminalSessionNotFoundException) {
            log.error("Session not found: $sessionId")
            protocol.close("Session not found")
            return
        }

        var terminalProcess = terminalProcessManager.getProcess(sessionId)
        if (terminalProcess == null) {
            terminalProcess = terminalProcessManager.createProcess(
                sessionId,
                terminalSession.workingDirectory,
                terminalSession.shellType,
                terminalSession.terminalSize
            )
        }


        val channelCapacity = 1024
        val scope = CoroutineScope(Dispatchers.IO + SupervisorJob())
        val wsToPty = Channel<String>(channelCapacity)
        val ptyToWs = Channel<String>(channelCapacity)

        val readWsJob = scope.launch {
            while (true) {
                val data = protocol.receive() ?: break
                wsToPty.send(data)
            }
        }

        scope.launch {
            for (command in wsToPty) {
                withContext(Dispatchers.IO) {
                    terminalProcess.write(command)
                }
            }
        }

        val readPtyJob = scope.launch {
            val buffer = ByteArray(1024)
            while (isActive) {
                val data = withContext(Dispatchers.IO) {
                    terminalProcess.inputStream().read(buffer)
                }
                if (data < 0) break
                if (data == 0) continue
                val slice = buffer.copyOf(data)
                val output = String(slice)
                ptyToWs.send(output)
            }
        }

        scope.launch {
            for (output in ptyToWs) {
                protocol.send(output)
            }
        }

        val finishSignal = CompletableDeferred<Job>()
        listOf(readWsJob, readPtyJob).forEach { job ->
            job.invokeOnCompletion {
                finishSignal.complete(job)
            }
        }

        try {
            finishSignal.await()
        } finally {
            protocol.close("Session finished")
            wsToPty.close()
            ptyToWs.close()

            runCatching {
                protocol.close()
            }

            runCatching {
                terminalProcess.terminate()
            }

            scope.cancel("Session finished")
        }

    }
}