package dev.waylon.terminal.infrastructure.config

import io.ktor.http.HttpStatusCode
import io.ktor.server.application.Application
import io.ktor.server.application.install
import io.ktor.server.application.log
import io.ktor.server.plugins.autohead.AutoHeadResponse
import io.ktor.server.plugins.statuspages.StatusPages
import io.ktor.server.response.respondText
import io.ktor.server.routing.get
import io.ktor.server.routing.routing


fun Application.configureRouting() {
    install(AutoHeadResponse)
    install(StatusPages) {
        exception<Throwable> { call, cause ->
            // Log detailed error stack trace using Ktor's logging system
            call.application.log.error("Internal Server Error: ${cause.message}", cause)
            call.respondText(text = "500: ${cause.message}", status = HttpStatusCode.InternalServerError)
        }
    }
    routing {
        get("/") {
            call.respondText("KT Terminal API")
        }
    }
}
