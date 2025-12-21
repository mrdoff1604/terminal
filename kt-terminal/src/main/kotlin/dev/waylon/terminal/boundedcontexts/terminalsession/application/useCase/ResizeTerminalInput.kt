package dev.waylon.terminal.boundedcontexts.terminalsession.application.useCase

import dev.waylon.terminal.boundedcontexts.terminalsession.infrastructure.dto.ResizeTerminalRequest

/**
 * Input data class for ResizeTerminalUseCase
 * Encapsulates all required parameters for resizing a terminal
 */
data class ResizeTerminalInput(
    val sessionId: String,
    val request: ResizeTerminalRequest
)