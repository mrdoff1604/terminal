package dev.waylon.terminal.boundedcontexts.terminalsession.application.service

import dev.waylon.terminal.boundedcontexts.terminalsession.application.process.TerminalProcessManager
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSession
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSessionFactory
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSessionRepository
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSessionStatus
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSize
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.model.TerminalConfig
import org.slf4j.LoggerFactory

/**
 * 终端会话服务
 * 负责协调终端会话的生命周期管理
 * 符合DDD最佳实践：服务层只负责协调，不包含业务逻辑
 * 符合DIP：依赖于抽象，不依赖于具体实现
 */
class TerminalSessionService(
    private val terminalConfig: TerminalConfig,
    private val terminalSessionRepository: TerminalSessionRepository,
    private val terminalProcessManager: TerminalProcessManager,
    private val terminalSessionFactory: TerminalSessionFactory
) {
    private val log = LoggerFactory.getLogger(TerminalSessionService::class.java)

    /**
     * 创建终端会话
     * @param userId 用户ID
     * @param title 会话标题
     * @param workingDirectory 工作目录
     * @param shellType shell类型
     * @param size 终端尺寸
     * @return 创建的终端会话
     */
    fun createSession(
        userId: String,
        title: String?,
        shellType: String?,
        workingDirectory: String?,
        size: TerminalSize?
    ): TerminalSession {
        // 使用工厂创建会话，工厂负责处理参数优先级
        val session = terminalSessionFactory.createSession(
            userId = userId,
            title = title,
            workingDirectory = workingDirectory,
            shellType = shellType,
            terminalSize = size
        )
        
        log.debug("TerminalSession created. {}", session)
        terminalSessionRepository.save(session)

        return session
    }

    /**
     * 根据ID获取终端会话
     * @param id 会话ID
     * @return 终端会话，如果不存在则返回null
     */
    fun getSessionById(id: String): TerminalSession? {
        return terminalSessionRepository.getById(id)?.also {
            updateSessionActivity(it)
        }
    }

    /**
     * 根据用户ID获取终端会话列表
     * @param userId 用户ID
     * @return 终端会话列表
     */
    fun getSessionsByUserId(userId: String): List<TerminalSession> {
        return terminalSessionRepository.getByUserId(userId)
    }

    /**
     * 获取所有终端会话
     * @return 所有终端会话列表
     */
    fun getAllSessions(): List<TerminalSession> {
        return terminalSessionRepository.getAll()
    }

    /**
     * 调整终端大小
     * @param id 会话ID
     * @param columns 列数
     * @param rows 行数
     * @return 调整后的终端会话，如果不存在则返回null
     */
    fun resizeTerminal(id: String, columns: Int, rows: Int): TerminalSession? {
        val terminalSize = TerminalSize(columns, rows)
        
        // 1. 先更新PTY进程的大小
        val resizeSuccess = terminalProcessManager.resizeProcess(id, terminalSize)
        log.debug("Resize PTY process result for session {}: {}", id, resizeSuccess)
        
        // 2. 然后更新DB中的会话对象
        return terminalSessionRepository.getById(id)?.also {
            it.resize(columns, rows)
            terminalSessionRepository.update(it)
        }
    }

    /**
     * 终止终端会话
     * @param id 会话ID
     * @param reason 终止原因
     * @return 终止的终端会话，如果不存在则返回null
     */
    fun terminateSession(id: String, reason: String? = null): TerminalSession? {
        return terminalSessionRepository.getById(id)?.also {
            // 使用领域模型的terminate方法
            it.terminate()

            // 从存储中移除
            terminalSessionRepository.deleteById(id)
        }
    }

    /**
     * 更新终端会话状态
     * @param id 会话ID
     * @param status 新状态
     * @return 更新后的终端会话，如果不存在则返回null
     */
    fun updateSessionStatus(id: String, status: TerminalSessionStatus): TerminalSession? {
        return terminalSessionRepository.getById(id)?.also {
            // 使用领域模型的updateStatus方法
            it.updateStatus(status)
            terminalSessionRepository.update(it)
        }
    }

    /**
     * 删除终端会话
     * @param id 会话ID
     * @return 是否删除成功
     */
    fun deleteSession(id: String): Boolean {
        val session = terminalSessionRepository.deleteById(id)
        return session != null
    }

    /**
     * 更新会话活动时间
     * @param session 终端会话
     */
    private fun updateSessionActivity(session: TerminalSession) {
        val now = System.currentTimeMillis()

        // 使用领域模型的方法更新活动时间和过期时间
        session.updateActivity(now)
        session.updateExpiryTime(terminalConfig.sessionTimeoutMs, now)

        // 更新存储中的会话
        terminalSessionRepository.update(session)
    }
}
