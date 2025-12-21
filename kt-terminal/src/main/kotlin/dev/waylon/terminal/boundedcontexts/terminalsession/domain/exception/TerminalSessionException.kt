package dev.waylon.terminal.boundedcontexts.terminalsession.domain.exception

/**
 * Base exception for terminal session-related errors
 * Extends RuntimeException to follow DDD best practices for domain exceptions
 */
open class TerminalSessionException(message: String, cause: Throwable? = null) : RuntimeException(message, cause)

/**
 * Exception thrown when a terminal session is not found
 */
class TerminalSessionNotFoundException(sessionId: String) : TerminalSessionException("Terminal session not found: $sessionId")

/**
 * Exception thrown when a terminal session is already terminated
 */
class TerminalSessionAlreadyTerminatedException(sessionId: String) : TerminalSessionException("Terminal session already terminated: $sessionId")

/**
 * Exception thrown when a terminal session operation is invalid
 */
class InvalidTerminalSessionOperationException(message: String) : TerminalSessionException(message)

/**
 * Exception thrown when terminal session parameters are invalid
 */
class InvalidTerminalSessionParametersException(message: String) : TerminalSessionException(message)