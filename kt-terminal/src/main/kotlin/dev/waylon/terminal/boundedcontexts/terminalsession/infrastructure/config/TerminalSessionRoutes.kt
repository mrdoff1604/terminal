package dev.waylon.terminal.boundedcontexts.terminalsession.infrastructure.config

import dev.waylon.terminal.boundedcontexts.terminalsession.application.service.TerminalSessionService
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
 */
fun Application.configureTerminalSessionRoutes() {
    val log = this.log
    val terminalSessionService by inject<TerminalSessionService>()

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

                    log.debug(
                        "Session creation parameters: userId={}, title={}, workingDirectory={}, shellType={}, columns={}, rows={}",
                        userId, title, workingDirectory, shellType, columnsParam, rowsParam
                    )

                    val terminalSize = if (columnsParam != null && rowsParam != null) {
                        TerminalSize(columnsParam.toInt(), rowsParam.toInt())
                    } else {
                        TerminalSize(80, 24)
                    }

                    val session = terminalSessionService.createSession(
                        userId,
                        title,
                        workingDirectory,
                        shellType,
                        terminalSize
                    )

                    log.info(
                        "Created new terminal session: {}, shellType: {}, workingDirectory: {}",
                        session.id, session.shellType, session.workingDirectory
                    )

                    call.respond(HttpStatusCode.Created, session)
                }

                // Get all sessions
                get {
                    log.debug("Getting all terminal sessions")
                    val sessions = terminalSessionService.getAllSessions()
                    log.debug("Found {} terminal sessions", sessions.size)
                    call.respond(HttpStatusCode.OK, sessions)
                }

                // Get session by ID
                get("/{id}") {
                    val id = call.parameters["id"] ?: return@get call.respond(
                        HttpStatusCode.BadRequest,
                        "Invalid session ID"
                    )
                    log.debug("Getting session by ID: {}", id)
                    val session = terminalSessionService.getSessionById(id) ?: return@get call.respond(
                        HttpStatusCode.NotFound,
                        "Session not found"
                    )
                    log.debug("Found session: {}, status: {}", id, session.status)
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
                    val session = terminalSessionService.resizeTerminal(id, columns, rows) ?: return@post call.respond(
                        HttpStatusCode.NotFound,
                        "Session not found"
                    )
                    // 使用专门的数据类响应，直接使用TerminalSize对象
                    val response = TerminalResizeResponse(
                        sessionId = session.id,
                        terminalSize = session.terminalSize,
                        status = session.status.toString()
                    )
                    log.debug("Resized terminal session {} successfully", id)
                    call.respond(HttpStatusCode.OK, response)
                }


                // Terminate session
                delete("/{id}") {
                    val id = call.parameters["id"] ?: return@delete call.respond(
                        HttpStatusCode.BadRequest,
                        "Invalid session ID"
                    )
                    log.debug("Terminating terminal session: {}", id)
                    val session = terminalSessionService.terminateSession(id) ?: return@delete call.respond(
                        HttpStatusCode.NotFound,
                        "Session not found"
                    )

                    val response = TerminalTerminateResponse(
                        sessionId = session.id,
                        reason = "User terminated",
                        status = session.status.toString()
                    )
                    log.info("Terminated terminal session: {}", id)
                    call.respond(HttpStatusCode.OK, response)
                }

            }
        }
    }
}
