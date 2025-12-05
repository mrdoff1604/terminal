package dev.waylon.terminal.boundedcontexts.terminalsession.application.service

import dev.waylon.terminal.boundedcontexts.terminalsession.application.process.TerminalProcessManager
import java.util.Date
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.cancel
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSession
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSessionRepository
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSessionStatus
import org.slf4j.LoggerFactory

/**
 * 会话过期管理器
 * 负责处理会话的过期检查和清理逻辑
 * 使用一个全局的定期检查协程，定期检查所有会话是否过期
 */
class TerminalSessionExpiryManager(
    private val terminalProcessManager: TerminalProcessManager? = null,
    private val terminalSessionRepository: TerminalSessionRepository? = null
) {
    private val logger = LoggerFactory.getLogger(TerminalSessionExpiryManager::class.java)

    // 使用协程作用域和SupervisorJob来管理协程生命周期
    private val scope = CoroutineScope(SupervisorJob() + Dispatchers.Default)

    // 检查间隔时间（毫秒），默认为1分钟
    private val checkIntervalMs = 60000L

    init {
        // 在构造函数中启动全局的定期检查协程
        startPeriodicCheck()
    }

    /**
     * 启动全局的定期检查协程
     */
    private fun startPeriodicCheck() {
        scope.launch {
            logger.info("Starting periodic session expiry check, interval: {}ms", checkIntervalMs)
            
            while (true) {
                try {
                    // 检查所有会话是否过期
                    checkAllSessions()
                    
                    // 等待检查间隔
                    delay(checkIntervalMs)
                } catch (e: Exception) {
                    logger.error("Error in periodic session expiry check: {}", e.message, e)
                    // 继续检查，不要因为错误而停止
                    delay(checkIntervalMs)
                }
            }
        }
    }

    /**
     * 检查所有会话是否过期
     */
    private fun checkAllSessions() {
        if (terminalSessionRepository == null) {
            logger.debug("TerminalSessionRepository is null, skipping session expiry check")
            return
        }
        
        val now = System.currentTimeMillis()
        logger.debug("Checking all sessions for expiry at: {}", Date(now))
        
        // 获取所有会话
        val allSessions = terminalSessionRepository.getAll()
        logger.debug("Found {} sessions to check", allSessions.size)
        
        // 检查每个会话是否过期
        allSessions.forEach { session ->
            if (session.status == TerminalSessionStatus.ACTIVE && session.isExpired(now)) {
                // 会话已过期，清理它
                cleanupExpiredSession(session)
            }
        }
    }

    /**
     * 清理单个过期会话
     */
    private fun cleanupExpiredSession(session: TerminalSession) {
        try {
            logger.info(
                "Cleaning up expired session: {}, expired at: {}",
                session.id,
                Date(session.expiredAt!!)
            )

            // 使用领域模型的terminate()方法更新状态
            session.terminate()

            // 清理相关资源
            terminalProcessManager?.terminateProcess(session.id)

            // 从存储中移除会话
            terminalSessionRepository?.deleteById(session.id)
        } catch (e: Exception) {
            logger.error("Error cleaning up session {}: {}", session.id, e.message, e)
        }
    }

    /**
     * 关闭过期管理器，清理所有资源
     */
    fun shutdown() {
        // 取消所有协程
        scope.cancel()
        logger.info("Session expiry manager shutdown completed")
    }
}
