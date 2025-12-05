package dev.waylon.terminal.boundedcontexts.terminalsession.infrastructure.config

import dev.waylon.terminal.boundedcontexts.terminalsession.application.process.TerminalProcessManager
import dev.waylon.terminal.boundedcontexts.terminalsession.application.service.TerminalCommunicationService
import dev.waylon.terminal.boundedcontexts.terminalsession.application.service.TerminalSessionService
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.InMemoryTerminalSessionRepository
import dev.waylon.terminal.boundedcontexts.terminalsession.domain.TerminalSessionRepository
import dev.waylon.terminal.boundedcontexts.terminalsession.infrastructure.service.Pty4jTerminalProcessManager
import dev.waylon.terminal.boundedcontexts.terminalsession.infrastructure.service.TerminalConfigService
import org.koin.dsl.module

// Export the terminal session module for use in Koin configuration
val terminalSessionModule = module {

    // Terminal configuration
    single { TerminalConfigService(get()).loadConfig() }

    // Session storage
    single<TerminalSessionRepository> { InMemoryTerminalSessionRepository() }

    single<TerminalSessionService> { TerminalSessionService(get(), get()) }

    // Terminal process manager
    single<TerminalProcessManager> { Pty4jTerminalProcessManager(get()) }

    single { TerminalCommunicationService(get(), get()) }

}
