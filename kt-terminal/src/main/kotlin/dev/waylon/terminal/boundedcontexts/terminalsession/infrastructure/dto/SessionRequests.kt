package dev.waylon.terminal.boundedcontexts.terminalsession.infrastructure.dto

import kotlinx.serialization.Serializable

/**
 * 创建终端会话请求体
 * 使用标准的JSON请求体替代Query String
 */
@Serializable
data class CreateSessionRequest(
    /**
     * 用户ID，必填字段
     */
    val userId: String,
    
    /**
     * 会话标题，可选字段
     */
    val title: String? = null,
    
    /**
     * 工作目录，可选字段
     */
    val workingDirectory: String? = null,
    
    /**
     * Shell类型，可选字段
     */
    val shellType: String? = null,
    
    /**
     * 终端列数，可选字段
     */
    val columns: Int? = null,
    
    /**
     * 终端行数，可选字段
     */
    val rows: Int? = null
)

/**
 * 调整终端大小请求体
 * 使用标准的JSON请求体替代Query String
 */
@Serializable
data class ResizeTerminalRequest(
    /**
     * 终端列数，必填字段
     */
    val columns: Int,
    
    /**
     * 终端行数，必填字段
     */
    val rows: Int
)
