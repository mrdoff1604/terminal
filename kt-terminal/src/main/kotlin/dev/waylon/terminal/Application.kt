package dev.waylon.terminal

import dev.waylon.terminal.boundedcontexts.terminalsession.infrastructure.config.configureTerminalSessionRoutes
import dev.waylon.terminal.boundedcontexts.terminalsession.infrastructure.config.configureTerminalWebSocketRoutes
import dev.waylon.terminal.boundedcontexts.terminalsession.infrastructure.config.configureTerminalWebTransportRoutes
import dev.waylon.terminal.infrastructure.config.configureHTTP
import dev.waylon.terminal.infrastructure.config.configureKoin
import dev.waylon.terminal.infrastructure.config.configureMonitoring
import dev.waylon.terminal.infrastructure.config.configureRouting
import dev.waylon.terminal.infrastructure.config.configureSerialization
import dev.waylon.terminal.infrastructure.config.installWebSockets
import io.ktor.server.application.Application

fun main(args: Array<String>) {
    io.ktor.server.netty.EngineMain.main(args)
}

fun Application.module() {
    configureKoin()
    configureMonitoring()
    configureHTTP()
    configureSerialization()
    installWebSockets()
    configureRouting()
    configureTerminalSessionRoutes()
    configureTerminalWebSocketRoutes()
    configureTerminalWebTransportRoutes()
}
