package dev.waylon.terminal.boundedcontexts.terminalsession.domain

import kotlin.time.Clock
import kotlin.time.Duration.Companion.milliseconds
import kotlin.time.Instant
import kotlinx.serialization.Serializable

/**
 * Terminal Session Aggregate Root
 * 
 * Represents a terminal session with all its lifecycle management capabilities.
 * This is the main aggregate root for the terminal session bounded context.
 * 
 * @property id Unique identifier for the session
 * @property userId User identifier associated with this session
 * @property title Optional title for the session
 * @property workingDirectory Initial working directory for the terminal
 * @property shellType Type of shell to use for this session
 * @property status Current status of the session
 * @property terminalSize Current size of the terminal
 * @property createdAt Session creation timestamp
 * @property updatedAt Last update timestamp
 * @property lastActiveTime Last activity timestamp
 * @property expiredAt Session expiry timestamp
 */
@Serializable
data class TerminalSession(
    val id: String,
    val userId: String,
    val title: String?,
    val workingDirectory: String,
    val shellType: String,
    var status: TerminalSessionStatus,
    var terminalSize: TerminalSize = TerminalSize(80, 24),
    val createdAt: Instant = Clock.System.now(),
    var updatedAt: Instant = Clock.System.now(),
    var lastActiveTime: Instant = Clock.System.now(),
    var expiredAt: Instant? = null
) {
    /**
     * Updates the session's last activity time and update timestamp.
     * 
     * @param now Current instant (defaults to now)
     * @return This session instance for method chaining
     */
    fun updateActivity(now: Instant = Clock.System.now()) = apply {
        lastActiveTime = now
        updatedAt = now
    }

    /**
     * Calculates and updates the session's expiry time based on the given timeout.
     * 
     * @param timeoutMs Timeout in milliseconds
     * @param now Current instant (defaults to now)
     * @return This session instance for method chaining
     */
    fun updateExpiryTime(timeoutMs: Long, now: Instant = Clock.System.now()) = apply {
        expiredAt = now + timeoutMs.milliseconds
        updatedAt = now
    }

    /**
     * Resizes the terminal to the specified dimensions.
     * 
     * @param columns New number of columns
     * @param rows New number of rows
     * @return This session instance for method chaining
     */
    fun resize(columns: Int, rows: Int) = apply {
        terminalSize = TerminalSize(columns, rows)
        updatedAt = Clock.System.now()
    }

    /**
     * Terminates the session, updating its status and timestamp.
     * 
     * @return This session instance for method chaining
     */
    fun terminate() = apply {
        status = TerminalSessionStatus.TERMINATED
        updatedAt = Clock.System.now()
    }

    /**
     * Checks if the session has expired based on its expiry time.
     * 
     * @param now Current instant (defaults to now)
     * @return True if session has expired, false otherwise
     */
    fun isExpired(now: Instant = Clock.System.now()) = expiredAt?.let { it < now } ?: false

    /**
     * Updates the session's status and timestamp.
     * 
     * @param newStatus New status for the session
     * @return This session instance for method chaining
     */
    fun updateStatus(newStatus: TerminalSessionStatus) = apply {
        status = newStatus
        updatedAt = Clock.System.now()
    }
}

/**
 * Terminal Session Status Enumeration
 * 
 * Defines the possible states a terminal session can be in.
 * Using an enum ensures compile-time safety when handling session statuses.
 */
@Serializable
enum class TerminalSessionStatus {
    /** Session is active and ready for use */
    ACTIVE,

    /** Session has been terminated */
    TERMINATED
}

/**
 * Terminal Size Value Object
 * 
 * Represents the dimensions of a terminal window.
 * This is an immutable value object with no identity.
 * 
 * @property cols Number of columns (characters wide)
 * @property rows Number of rows (lines tall)
 * @throws IllegalArgumentException if cols or rows are not positive
 */
@Serializable
data class TerminalSize(
    val cols: Int,
    val rows: Int
) {
    init {
        require(cols > 0) { "Columns must be greater than 0" }
        require(rows > 0) { "Rows must be greater than 0" }
    }
}
