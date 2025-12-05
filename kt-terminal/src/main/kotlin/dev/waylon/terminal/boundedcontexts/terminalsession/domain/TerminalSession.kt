package dev.waylon.terminal.boundedcontexts.terminalsession.domain

import kotlinx.serialization.Serializable

/**
 * 终端会话实体
 * 聚合根，管理终端会话的生命周期
 */
@Serializable
data class TerminalSession(
    val id: String,
    val userId: String,
    val title: String?,
    val workingDirectory: String,
    val shellType: String,
    var status: dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSessionStatus,
    var terminalSize: dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSize = TerminalSize(80, 24),
    val createdAt: Long = System.currentTimeMillis(),
    var updatedAt: Long = System.currentTimeMillis(),
    var lastActiveTime: Long = System.currentTimeMillis(),
    var expiredAt: Long?
) {
    /**
     * 更新会话活动时间
     */
    fun updateActivity(now: Long = System.currentTimeMillis()): TerminalSession {
        this.lastActiveTime = now
        this.updatedAt = now
        return this
    }
    
    /**
     * 计算并更新过期时间
     */
    fun updateExpiryTime(timeoutMs: Long, now: Long = System.currentTimeMillis()): TerminalSession {
        this.expiredAt = now + timeoutMs
        this.updatedAt = now
        return this
    }
    
    /**
     * 调整终端大小
     */
    fun resize(columns: Int, rows: Int): TerminalSession {
        this.terminalSize = TerminalSize(columns, rows)
        this.updatedAt = System.currentTimeMillis()
        return this
    }
    
    /**
     * 终止会话
     */
    fun terminate(): TerminalSession {
        this.status = TerminalSessionStatus.TERMINATED
        this.updatedAt = System.currentTimeMillis()
        return this
    }
    
    /**
     * 检查会话是否过期
     */
    fun isExpired(now: Long = System.currentTimeMillis()): Boolean {
        return this.expiredAt?.let { it < now } ?: false
    }
    
    /**
     * 更新会话状态
     */
    fun updateStatus(newStatus: TerminalSessionStatus): TerminalSession {
        this.status = newStatus
        this.updatedAt = System.currentTimeMillis()
        return this
    }
}

/**
 * 终端会话状态
 * 使用密封类，确保编译时检查所有状态
 */
@Serializable
enum class TerminalSessionStatus {
    ACTIVE,
    TERMINATED
}

/**
 * 终端尺寸值对象
 * 不可变，没有标识
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
