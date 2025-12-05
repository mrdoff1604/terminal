package dev.waylon.terminal.boundedcontexts.terminalsession.infrastructure.config

import dev.waylon.terminal.boundedcontexts.terminalsession.application.useCase.CreateTerminalSessionUseCase
import dev.waylon.terminal.boundedcontexts.terminalsession.application.useCase.GetAllTerminalSessionsUseCase
import dev.waylon.terminal.boundedcontexts.terminalsession.application.useCase.GetTerminalSessionByIdUseCase
import dev.waylon.terminal.boundedcontexts.terminalsession.application.useCase.ResizeTerminalUseCase
import dev.waylon.terminal.boundedcontexts.terminalsession.application.useCase.TerminateTerminalSessionUseCase
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSize
import dev.waylon.terminal.boundedcontexts.terminalsession.infrastructure.dto.CreateSessionRequest
import dev.waylon.terminal.boundedcontexts.terminalsession.infrastructure.dto.ResizeTerminalRequest
import io.ktor.http.HttpStatusCode
import io.ktor.server.application.Application
import io.ktor.server.application.log
import io.ktor.server.request.receive
import io.ktor.server.response.respond
import io.ktor.server.routing.delete
import io.ktor.server.routing.get
import io.ktor.server.routing.post
import io.ktor.server.routing.route
import io.ktor.server.routing.routing
import kotlinx.serialization.Serializable
import kotlinx.serialization.SerializationException
import org.koin.ktor.ext.inject

// 响应数据类

@Serializable
data class TerminalResizeResponse(
    val sessionId: String,
    val terminalSize: TerminalSize,
    val status: String
)

@Serializable
data class TerminalInterruptResponse(
    val sessionId: String,
    val status: String
)

@Serializable
data class TerminalTerminateResponse(
    val sessionId: String,
    val reason: String,
    val status: String
)

@Serializable
data class TerminalStatusResponse(
    val status: String
)

/**
 * Terminal session routes configuration
 * This follows the same pattern as other route configurations
 * Route层只负责处理HTTP请求和响应，业务逻辑封装在UseCase中
 */
fun Application.configureTerminalSessionRoutes() {
    val log = this.log
    
    // 注入UseCase，替代直接注入服务层
    val createTerminalSessionUseCase by inject<CreateTerminalSessionUseCase>()
    val getAllTerminalSessionsUseCase by inject<GetAllTerminalSessionsUseCase>()
    val getTerminalSessionByIdUseCase by inject<GetTerminalSessionByIdUseCase>()
    val resizeTerminalUseCase by inject<ResizeTerminalUseCase>()
    val terminateTerminalSessionUseCase by inject<TerminateTerminalSessionUseCase>()

    routing {
        // API routes with /api prefix
        route("/api") {
            route("/sessions") {
                // Create a new session
                post {
                    log.debug("Creating new terminal session")
                    try {
                        // 接收请求体
                        val request = call.receive<CreateSessionRequest>()
                        
                        // 使用UseCase执行业务逻辑
                        val session = createTerminalSessionUseCase.execute(request)
                        
                        call.respond(HttpStatusCode.Created, session)
                    } catch (e: SerializationException) {
                        // 请求格式错误
                        log.error("Invalid request format: {}", e.message)
                        call.respond(HttpStatusCode.BadRequest, "Invalid request format")
                    } catch (e: IllegalArgumentException) {
                        // 参数验证失败
                        log.error("Validation failed: {}", e.message)
                        call.respond(HttpStatusCode.BadRequest, mapOf("error" to e.message))
                    } catch (e: Exception) {
                        // 其他异常
                        log.error("Error creating session: {}", e.message, e)
                        call.respond(HttpStatusCode.InternalServerError, mapOf("error" to "Failed to create session"))
                    }
                }

                // Get all sessions
                get {
                    log.debug("Getting all terminal sessions")
                    try {
                        // 使用UseCase执行业务逻辑
                        val sessions = getAllTerminalSessionsUseCase.execute()
                        
                        call.respond(HttpStatusCode.OK, sessions)
                    } catch (e: Exception) {
                        log.error("Error getting sessions: {}", e.message, e)
                        call.respond(HttpStatusCode.InternalServerError, mapOf("error" to "Failed to get sessions"))
                    }
                }

                // Get session by ID
                get("/{id}") {
                    val id = call.parameters["id"] ?: return@get call.respond(
                        HttpStatusCode.BadRequest,
                        mapOf("error" to "Invalid session ID")
                    )
                    log.debug("Getting session by ID: {}", id)
                    try {
                        // 使用UseCase执行业务逻辑
                        val session = getTerminalSessionByIdUseCase.execute(sessionId = id) ?: return@get call.respond(
                            HttpStatusCode.NotFound,
                            mapOf("error" to "Session not found")
                        )
                        
                        call.respond(HttpStatusCode.OK, session)
                    } catch (e: Exception) {
                        log.error("Error getting session: {}", e.message, e)
                        call.respond(HttpStatusCode.InternalServerError, mapOf("error" to "Failed to get session"))
                    }
                }

                // Resize terminal
                post("/{id}/resize") {
                    val id = call.parameters["id"] ?: return@post call.respond(
                        HttpStatusCode.BadRequest,
                        "Invalid session ID"
                    )
                    
                    try {
                        // 接收请求体
                        val request = call.receive<ResizeTerminalRequest>()
                        
                        log.debug("Resizing terminal session {} to columns: {}, rows: {}", id, request.columns, request.rows)
                        
                        // 使用UseCase执行业务逻辑
                        val session = resizeTerminalUseCase.execute(sessionId = id, request = request) ?: return@post call.respond(
                            HttpStatusCode.NotFound,
                            "Session not found"
                        )
                        
                        // 使用专门的数据类响应，直接使用TerminalSize对象
                        val response = TerminalResizeResponse(
                            sessionId = session.id,
                            terminalSize = session.terminalSize,
                            status = session.status.toString()
                        )
                        
                        call.respond(HttpStatusCode.OK, response)
                    } catch (e: SerializationException) {
                        // 请求格式错误
                        log.error("Invalid request format: {}", e.message)
                        call.respond(HttpStatusCode.BadRequest, mapOf("error" to "Invalid request format"))
                    } catch (e: IllegalArgumentException) {
                        // 参数验证失败
                        log.error("Validation failed: {}", e.message)
                        call.respond(HttpStatusCode.BadRequest, mapOf("error" to e.message))
                    } catch (e: Exception) {
                        // 其他异常
                        log.error("Error resizing terminal: {}", e.message, e)
                        call.respond(HttpStatusCode.InternalServerError, mapOf("error" to "Failed to resize terminal"))
                    }
                }

                // Terminate session
                delete("/{id}") {
                    val id = call.parameters["id"] ?: return@delete call.respond(
                        HttpStatusCode.BadRequest,
                        mapOf("error" to "Invalid session ID")
                    )
                    log.debug("Terminating terminal session: {}", id)
                    try {
                        // 使用UseCase执行业务逻辑
                        val session = terminateTerminalSessionUseCase.execute(sessionId = id) ?: return@delete call.respond(
                            HttpStatusCode.NotFound,
                            mapOf("error" to "Session not found")
                        )

                        val response = TerminalTerminateResponse(
                            sessionId = session.id,
                            reason = "User terminated",
                            status = session.status.toString()
                        )
                        
                        call.respond(HttpStatusCode.OK, response)
                    } catch (e: Exception) {
                        log.error("Error terminating session: {}", e.message, e)
                        call.respond(HttpStatusCode.InternalServerError, mapOf("error" to "Failed to terminate session"))
                    }
                }
            }
        }
    }
}
