package dev.waylon.terminal.infrastructure.config

import dev.waylon.terminal.boundedcontexts.terminalsession.infrastructure.config.terminalSessionModule
import io.ktor.server.application.Application
import io.ktor.server.application.install
import org.koin.dsl.module
import org.koin.ktor.plugin.Koin

/**
 * Koin Dependency Injection Configuration
 * Configures Koin DI framework with all application modules
 */
fun Application.configureKoin() {
    install(Koin) {
        // Add application instance to Koin container
        modules(
            module {
                single { this@configureKoin }
            },
            terminalSessionModule
        )
    }
}