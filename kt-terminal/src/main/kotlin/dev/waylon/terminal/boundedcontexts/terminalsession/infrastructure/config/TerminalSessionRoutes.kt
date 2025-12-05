package dev.waylon.terminal.boundedcontexts.terminalsession.infrastructure.config

import dev.waylon.terminal.boundedcontexts.terminalsession.application.useCase.CreateTerminalSessionUseCase
import dev.waylon.terminal.boundedcontexts.terminalsession.application.useCase.GetAllTerminalSessionsUseCase
import dev.waylon.terminal.boundedcontexts.terminalsession.application.useCase.GetTerminalSessionByIdUseCase
import dev.waylon.terminal.boundedcontexts.terminalsession.application.useCase.ResizeTerminalUseCase
import dev.waylon.terminal.boundedcontexts.terminalsession.application.useCase.TerminateTerminalSessionUseCase
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSize
import io.ktor.http.HttpStatusCode
import io.ktor.server.application.Application
import io.ktor.server.application.log
import io.ktor.server.response.respond
import io.ktor.server.routing.delete
import io.ktor.server.routing.get
import io.ktor.server.routing.post
import io.ktor.server.routing.route
import io.ktor.server.routing.routing
import kotlinx.serialization.Serializable
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

                    val userId = call.request.queryParameters["userId"] ?: return@post call.respond(
                        HttpStatusCode.BadRequest,
                        "Missing userId"
                    )
                    val title = call.request.queryParameters["title"]
                    val workingDirectory = call.request.queryParameters["workingDirectory"]
                    val shellType = call.request.queryParameters["shellType"]
                    val columnsParam = call.request.queryParameters["columns"]
                    val rowsParam = call.request.queryParameters["rows"]

                    // 使用UseCase执行业务逻辑
                    val session = createTerminalSessionUseCase.execute(
                        userId = userId,
                        title = title,
                        workingDirectory = workingDirectory,
                        shellType = shellType,
                        columns = columnsParam,
                        rows = rowsParam
                    )

                    call.respond(HttpStatusCode.Created, session)
                }

                // Get all sessions
                get {
                    log.debug("Getting all terminal sessions")
                    
                    // 使用UseCase执行业务逻辑
                    val sessions = getAllTerminalSessionsUseCase.execute()
                    
                    call.respond(HttpStatusCode.OK, sessions)
                }

                // Get session by ID
                get("/{id}") {
                    val id = call.parameters["id"] ?: return@get call.respond(
                        HttpStatusCode.BadRequest,
                        "Invalid session ID"
                    )
                    log.debug("Getting session by ID: {}", id)
                    
                    // 使用UseCase执行业务逻辑
                    val session = getTerminalSessionByIdUseCase.execute(sessionId = id) ?: return@get call.respond(
                        HttpStatusCode.NotFound,
                        "Session not found"
                    )
                    
                    call.respond(HttpStatusCode.OK, session)
                }

                // Resize terminal
                post("/{id}/resize") {
                    val id = call.parameters["id"] ?: return@post call.respond(
                        HttpStatusCode.BadRequest,
                        "Invalid session ID"
                    )
                    val columns = call.request.queryParameters["cols"]?.toIntOrNull() ?: return@post call.respond(
                        HttpStatusCode.BadRequest,
                        "Missing or invalid columns"
                    )
                    val rows = call.request.queryParameters["rows"]?.toIntOrNull() ?: return@post call.respond(
                        HttpStatusCode.BadRequest,
                        "Missing or invalid rows"
                    )

                    log.debug("Resizing terminal session {} to columns: {}, rows: {}", id, columns, rows)
                    
                    // 使用UseCase执行业务逻辑
                    val session = resizeTerminalUseCase.execute(sessionId = id, columns = columns, rows = rows) ?: return@post call.respond(
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
                }

                // Terminate session
                delete("/{id}") {
                    val id = call.parameters["id"] ?: return@delete call.respond(
                        HttpStatusCode.BadRequest,
                        "Invalid session ID"
                    )
                    log.debug("Terminating terminal session: {}", id)
                    
                    // 使用UseCase执行业务逻辑
                    val session = terminateTerminalSessionUseCase.execute(sessionId = id) ?: return@delete call.respond(
                        HttpStatusCode.NotFound,
                        "Session not found"
                    )

                    val response = TerminalTerminateResponse(
                        sessionId = session.id,
                        reason = "User terminated",
                        status = session.status.toString()
                    )
                    
                    call.respond(HttpStatusCode.OK, response)
                }
            }
        }
    }
}
